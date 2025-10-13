#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;
mod vial;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::flash::{Async, Flash};
use embassy_rp::gpio::{Flex, Input, Level};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Duration;
use log::info;
use keymap::{COL, ROW};
use rmk::action::EncoderAction;
use rmk::bidirectional_matrix::{BidirectionalMatrix, ScanLocation};
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, KeyboardUsbConfig, RmkConfig, StorageConfig, TapHoldConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join3;
use rmk::input_device::rotary_encoder::{RotaryEncoder};
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::{initialize_encoder_keymap_and_storage, run_devices, run_rmk};
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

use {panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;
const PIN_NUM: usize = 10;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {    
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    let flex_pins =
        config_matrix_pins_rp!(peripherals: p, pins: [
            // Rows Index 0-3
            PIN_26, PIN_15, PIN_14, PIN_13,
            // Cols Index 4-9
            PIN_29, PIN_28, PIN_27, PIN_12, PIN_11, PIN_10
        ]);
    
    let scan_map = [
        // Row 1: Left
        [ScanLocation::Pins(0,4), ScanLocation::Pins(4,0), ScanLocation::Pins(0,5), ScanLocation::Pins(5,0), ScanLocation::Pins(0,6), ScanLocation::Pins(6,0),
        // Row 1: Right
        ScanLocation::Pins(7,0), ScanLocation::Pins(0,7), ScanLocation::Pins(8,0), ScanLocation::Pins(0,8), ScanLocation::Pins(9,0), ScanLocation::Pins(0,9)],
        // Row 2: Left
        [ScanLocation::Pins(1,4), ScanLocation::Pins(4,1), ScanLocation::Pins(1,5), ScanLocation::Pins(5,1), ScanLocation::Pins(1,6), ScanLocation::Pins(6,1),
        // Row 2: Right
        ScanLocation::Pins(7,1), ScanLocation::Pins(1,7), ScanLocation::Pins(8,1), ScanLocation::Pins(1,8), ScanLocation::Pins(9,1), ScanLocation::Pins(1,9)],
        // Row 3: Left
        [ScanLocation::Pins(2,4), ScanLocation::Pins(4,2), ScanLocation::Pins(2,5), ScanLocation::Pins(5,2), ScanLocation::Pins(2,6), ScanLocation::Pins(6,2),
        // Row 3: Right
        ScanLocation::Pins(7,2), ScanLocation::Pins(2,7), ScanLocation::Pins(8,2), ScanLocation::Pins(2,8), ScanLocation::Pins(9,2), ScanLocation::Pins(2,9)],
        // Row 4: Left
        [ScanLocation::Pins(3,4), ScanLocation::Pins(4,3), ScanLocation::Pins(3,5), ScanLocation::Pins(5,3), ScanLocation::Ignore, ScanLocation::Ignore,
        // Row 4: Right
        ScanLocation::Ignore, ScanLocation::Ignore, ScanLocation::Pins(8,3), ScanLocation::Pins(3,8), ScanLocation::Pins(9,3), ScanLocation::Pins(3,9)],
    ];
    
    // Use internal flash to emulate eeprom
    // Both blocking and async flash are support, use different API
    // let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(p.FLASH);
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "archnode",
        product_name: "Weaver v2",
        serial_number: "vial:f64c2b3c:000001",
    };

    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]);

    let rmk_config = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        ..Default::default()
    };

    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let storage_config = StorageConfig::default();
    let mut behavior_config = BehaviorConfig {
      tap_hold: TapHoldConfig {
          enable_hrm: true,
          timeout: Duration::from_millis(165),
          ..Default::default()
      },
      ..Default::default()  
    };
    let mut encoder_map: [[EncoderAction; _]; _] = keymap::get_default_encoder_map();
    let (keymap, mut storage) =
        initialize_encoder_keymap_and_storage(&mut default_keymap, &mut encoder_map, flash, &storage_config, &mut behavior_config).await;

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<PIN_NUM, PIN_NUM>::new();
    let mut matrix = BidirectionalMatrix::<_, _, PIN_NUM, ROW, COL>::new(flex_pins, debouncer, scan_map);
    let mut keyboard = Keyboard::new(&keymap);
    
    // Initialize Rotary Encoder
    let pin_a = Input::new(p.PIN_1, embassy_rp::gpio::Pull::Up);
    let pin_b = Input::new(p.PIN_2, embassy_rp::gpio::Pull::Up);
    let mut encoder = RotaryEncoder::with_resolution(pin_a, pin_b, 1, false, 0);

    // Start
    join3(
        run_devices! (
            (matrix, encoder) => EVENT_CHANNEL,
        ),
        keyboard.run(),
        run_rmk(&keymap, driver, &mut storage, rmk_config),
    )
    .await;
}

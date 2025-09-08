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
use log::info;
use keymap::{COL, ROW};
use rmk::bidirectional_matrix::{BidirectionalMatrix, ScanLocation};
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, KeyboardUsbConfig, RmkConfig, StorageConfig, VialConfig};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join3;
use rmk::input_device::rotary_encoder::{DefaultPhase, RotaryEncoder};
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::{initialize_keymap_and_storage, run_devices, run_rmk};
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

use {panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const FLASH_SIZE: usize = 2 * 1024 * 1024;

const PIN_NUM: usize = 14;

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
            // Rows Index 0-7
            PIN_26, PIN_15, PIN_14, PIN_13, PIN_9, PIN_10, PIN_11, PIN_12,
            // Cols Index 8-13
            PIN_29, PIN_28, PIN_27, PIN_8, PIN_7, PIN_6
        ]);
    
    let scan_map = [
        // Row 1: Left
        [ScanLocation::Pins(0,8), ScanLocation::Pins(8,0), ScanLocation::Pins(0,9), ScanLocation::Pins(9,0), ScanLocation::Pins(0,10), ScanLocation::Pins(10,0),
        // Row 1: Right
        ScanLocation::Pins(11,4), ScanLocation::Pins(4,11), ScanLocation::Pins(12,4), ScanLocation::Pins(4,12), ScanLocation::Pins(13,4), ScanLocation::Pins(4,13)],
        // Row 2: Left
        [ScanLocation::Pins(1,8), ScanLocation::Pins(8,1), ScanLocation::Pins(1,9), ScanLocation::Pins(9,1), ScanLocation::Pins(1,10), ScanLocation::Pins(10,1),
        // Row 2: Right
        ScanLocation::Pins(11,5), ScanLocation::Pins(5,11), ScanLocation::Pins(12,5), ScanLocation::Pins(5,12), ScanLocation::Pins(13,5), ScanLocation::Pins(5,13)],
        // Row 3: Left
        [ScanLocation::Pins(2,8), ScanLocation::Pins(8,2), ScanLocation::Pins(2,9), ScanLocation::Pins(9,2), ScanLocation::Pins(2,10), ScanLocation::Pins(10,2),
        // Row 3: Right
        ScanLocation::Pins(11,6), ScanLocation::Pins(6,11), ScanLocation::Pins(12,6), ScanLocation::Pins(6,12), ScanLocation::Pins(13,6), ScanLocation::Pins(6,13)],
        // Row 4: Left
        [ScanLocation::Pins(3,8), ScanLocation::Pins(8,3), ScanLocation::Pins(3,9), ScanLocation::Pins(9,3), ScanLocation::Ignore, ScanLocation::Ignore,
        // Row 4: Right
        ScanLocation::Ignore, ScanLocation::Ignore, ScanLocation::Pins(12,7), ScanLocation::Pins(7,12), ScanLocation::Pins(13,7), ScanLocation::Pins(7,13)],
    ];
    
    // Use internal flash to emulate eeprom
    // Both blocking and async flash are support, use different API
    // let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(p.FLASH);
    let flash = Flash::<_, Async, FLASH_SIZE>::new(p.FLASH, p.DMA_CH0);

    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "archnode",
        product_name: "Weaver",
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
    let mut behavior_config = BehaviorConfig::default();
    let (keymap, mut storage) =
        initialize_keymap_and_storage(&mut default_keymap, flash, &storage_config, &mut behavior_config).await;

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<ROW, COL>::new();
    let mut matrix = BidirectionalMatrix::<_, _, PIN_NUM, ROW, COL>::new(flex_pins, debouncer, scan_map);
    let mut keyboard = Keyboard::new(&keymap);
    
    // Initialize Rotary Encoder
    let pin_a = Input::new(p.PIN_1, embassy_rp::gpio::Pull::None);
    let pin_b = Input::new(p.PIN_2, embassy_rp::gpio::Pull::None);
    let mut encoder = RotaryEncoder::with_phase(pin_a, pin_b, DefaultPhase, 1);

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

macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, pins: [$($flex_pin:ident), *]) => {
        {
            let mut flex_pins = [$(embassy_rp::gpio::Flex::new($p.$flex_pin)), +];
            flex_pins.iter_mut().for_each(|p| {
                p.set_as_input();
                p.set_level(embassy_rp::gpio::Level::Low);
            });
            (flex_pins)
        }
    };
}

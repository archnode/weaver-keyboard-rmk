macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, pins: [$($flex_pin:ident), *]) => {
        {
            let mut flex_pins = [$(Flex::new($p.$flex_pin)), +];
            flex_pins.iter_mut().for_each(|p| {
                p.set_as_input();
                p.set_level(Level::Low);
            });
            (flex_pins)
        }
    };
}


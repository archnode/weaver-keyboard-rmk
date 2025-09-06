macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Flex::new($p.$out_pin)), +];
            let mut input_pins = [$(Flex::new($p.$in_pin)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            input_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
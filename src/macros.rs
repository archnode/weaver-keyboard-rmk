macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Flex::new($p.$out_pin)), +];
            let mut input_pins = [$(Flex::new($p.$in_pin)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_as_output();
                p.set_level(Level::Low);
                p.set_pull(Pull::Down);
            });
            input_pins.iter_mut().for_each(|p| {
                p.set_as_input();
                p.set_level(Level::Low);
                p.set_pull(Pull::Down);
            });
            (input_pins, output_pins)
        }
    };
}
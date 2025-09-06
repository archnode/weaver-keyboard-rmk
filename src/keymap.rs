
use rmk::{a, action::KeyAction, k, layer, mo};
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(AudioVolUp), k!(B), k!(AudioVolDown), a!(No), a!(No), a!(No)],
            [k!(Kp4), k!(LShift), k!(Kp6), a!(No), a!(No), a!(No)],
            [mo!(1), k!(Kp2), k!(Kp3), a!(No), a!(No), a!(No)],
            [mo!(1), a!(No), k!(Kp0), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [k!(Kp7), k!(Kp8), k!(Kp9), a!(No), a!(No), a!(No)],
            [k!(Kp4), k!(LCtrl), k!(Kp6), a!(No), a!(No), a!(No)],
            [mo!(1), k!(Kp2), k!(Kp3), a!(No), a!(No), a!(No)],
            [mo!(1), a!(No), k!(Kp0), a!(No), a!(No), a!(No)]
        ]),
    ]
}
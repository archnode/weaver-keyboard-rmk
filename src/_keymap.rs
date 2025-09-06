use rmk::action::KeyAction;
use rmk::keycode::ModifierCombination;
use rmk::{a, k, layer, mo, wm};
pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Escape), k!(J), k!(D), k!(U), k!(A), k!(X), k!(P), k!(H), k!(L), k!(M), k!(W), k!(Minus)],
            [k!(Tab), k!(C), wm!(T, ModifierCombination::new_from(true, false, true, false, false)), wm!(I, ModifierCombination::new_from(false, false, false, false, true)), k!(E), k!(O), k!(B), k!(N), wm!(R, ModifierCombination::new_from(false, false, false, false, true)), wm!(S, ModifierCombination::new_from(false, false, true, false, false)), k!(G), k!(Q)],
            [k!(Backspace), k!(F), k!(V), k!(LeftBracket), k!(Quote), wm!(Semicolon, ModifierCombination::new_from(false, true, false, false, false)), wm!(Z, ModifierCombination::new_from(true, true, false, false, false)), k!(Y), k!(Comma), k!(Dot), k!(K), k!(Enter)],
            [k!(LShift), k!(Space), mo!(2), mo!(1), a!(No), a!(No), a!(No), a!(No), mo!(3), a!(No), a!(No), a!(No)]
        ]),
    ]
}

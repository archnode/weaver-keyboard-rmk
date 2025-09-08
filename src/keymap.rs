
use rmk::{a, action::KeyAction, k, keycode::ModifierCombination, layer, mo, wm, mt};
pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Escape), k!(J), k!(D), k!(U), k!(A), k!(X),
                k!(P), k!(H), k!(L), k!(M), k!(W), k!(Minus)],
            [k!(Tab), k!(C), mt!(T, ModifierCombination::new_from(false, false, true, false, false)), mt!(I, ModifierCombination::new_from(false, false, false, false, true)), k!(E), k!(O),
                k!(B), k!(N), mt!(R, ModifierCombination::new_from(false, false, false, false, true)), mt!(S, ModifierCombination::new_from(false, false, true, false, false)), k!(G), k!(Q)],
            [k!(Backspace), k!(F), k!(V), k!(LeftBracket), k!(Quote), mt!(Semicolon, ModifierCombination::new_from(false, true, false, false, false)),
                mt!(Z, ModifierCombination::new_from(true, true, false, false, false)), k!(Y), k!(Comma), k!(Dot), k!(K), k!(Enter)],
            [a!(No), a!(No), k!(LShift), k!(Space), a!(No), a!(No),
                a!(No), a!(No), mo!(2), mo!(1), a!(No), mo!(3)]
        ]),
        layer!([
            [k!(NonusHash), k!(Grave), wm!(Slash, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc8, ModifierCombination::new_from(true, false, true, false, false)), wm!(Kc9, ModifierCombination::new_from(true, false, true, false, false)), k!(Equal),
                wm!(Kc1, ModifierCombination::new_from(false, false, false, true, false)), k!(NonusBackslash), wm!(NonusBackslash, ModifierCombination::new_from(false, false, false, true, false)), wm!(NonusBackslash, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc9, ModifierCombination::new_from(false, false, false, true, false)), wm!(Grave, ModifierCombination::new_from(false, false, false, true, false))],
            [wm!(E, ModifierCombination::new_from(true, false, true, false, false)), wm!(Minus, ModifierCombination::new_from(true, false, true, false, false)), wm!(Kc7, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc7, ModifierCombination::new_from(true, false, true, false, false)), wm!(Kc0, ModifierCombination::new_from(true, false, true, false, false)), k!(RightBracket),
                wm!(Minus, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc8, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc9, ModifierCombination::new_from(false, false, false, true, false)), k!(Slash), wm!(Dot, ModifierCombination::new_from(false, false, false, true, false)), wm!(Q, ModifierCombination::new_from(true, false, true, false, false))],
            [a!(No), k!(NonusHash), wm!(Kc4, ModifierCombination::new_from(false, false, false, true, false)), wm!(NonusBackslash, ModifierCombination::new_from(true, false, true, false, false)), wm!(RightBracket, ModifierCombination::new_from(true, false, true, false, false)), wm!(Equal, ModifierCombination::new_from(false, false, false, true, false)),
                k!(RightBracket), wm!(Kc5, ModifierCombination::new_from(false, false, false, true, false)), wm!(Kc2, ModifierCombination::new_from(false, false, false, true, false)), wm!(Backslash, ModifierCombination::new_from(false, false, false, true, false)), wm!(Comma, ModifierCombination::new_from(false, false, false, true, false)), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
                a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            [a!(No), k!(PageUp), k!(Backspace), k!(Up), k!(Delete), k!(PageDown),
                a!(No), k!(Kc7), k!(Kc8), k!(Kc9), k!(KpPlus), k!(No)],
            [KeyAction::Transparent, k!(Home), k!(Left), k!(Down), k!(Right), k!(End),
                k!(No), k!(Kc4), k!(Kc5), k!(Kc6), k!(KpComma), k!(No)],
            [k!(No), k!(No), k!(No), k!(No), k!(No), KeyAction::Transparent,
                k!(Kc0), k!(Kc1), k!(Kc2), k!(Kc3), k!(KpEqual), k!(No)],
            [KeyAction::Transparent, KeyAction::Transparent, KeyAction::Transparent, k!(Home), k!(No), k!(No),
                k!(No), k!(No), KeyAction::Transparent, KeyAction::Transparent, KeyAction::Transparent, KeyAction::Transparent]
        ]),
        layer!([
            [k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6),
                k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12)],
            [k!(PrintScreen), k!(ScrollLock), k!(Pause), a!(No), k!(BrightnessUp), k!(BrightnessDown),
                k!(AudioMute), k!(AudioVolDown), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
                a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No),
                a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ])
    ]
}

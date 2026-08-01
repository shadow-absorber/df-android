use android_activity::input::{KeyEvent, Keycode};
use ruffle_core::events::{KeyDescriptor, KeyLocation, LogicalKey, NamedKey, PhysicalKey};

pub fn android_key_event_to_ruffle_key_descriptor(android: &KeyEvent) -> Option<KeyDescriptor> {
    // TODO: Maybe do something with `android.scan_code()`?
    let physical_key = PhysicalKey::Unknown;

    let logical_key = match android.key_code() {
        Keycode::DpadUp => LogicalKey::Named(NamedKey::ArrowUp),
        Keycode::DpadDown => LogicalKey::Named(NamedKey::ArrowDown),
        Keycode::DpadLeft => LogicalKey::Named(NamedKey::ArrowLeft),
        Keycode::DpadRight => LogicalKey::Named(NamedKey::ArrowRight),
        Keycode::Keycode0 => LogicalKey::Character('0'),
        Keycode::Keycode1 => LogicalKey::Character('1'),
        Keycode::Keycode2 => LogicalKey::Character('2'),
        Keycode::Keycode3 => LogicalKey::Character('3'),
        Keycode::Keycode4 => LogicalKey::Character('4'),
        Keycode::Keycode5 => LogicalKey::Character('5'),
        Keycode::Keycode6 => LogicalKey::Character('6'),
        Keycode::Keycode7 => LogicalKey::Character('7'),
        Keycode::Keycode8 => LogicalKey::Character('8'),
        Keycode::Keycode9 => LogicalKey::Character('9'),
        Keycode::A => LogicalKey::Character('a'),
        Keycode::B => LogicalKey::Character('b'),
        Keycode::C => LogicalKey::Character('c'),
        Keycode::D => LogicalKey::Character('d'),
        Keycode::E => LogicalKey::Character('e'),
        Keycode::F => LogicalKey::Character('f'),
        Keycode::G => LogicalKey::Character('g'),
        Keycode::H => LogicalKey::Character('h'),
        Keycode::I => LogicalKey::Character('i'),
        Keycode::J => LogicalKey::Character('j'),
        Keycode::K => LogicalKey::Character('k'),
        Keycode::L => LogicalKey::Character('l'),
        Keycode::M => LogicalKey::Character('m'),
        Keycode::N => LogicalKey::Character('n'),
        Keycode::O => LogicalKey::Character('o'),
        Keycode::P => LogicalKey::Character('p'),
        Keycode::Q => LogicalKey::Character('q'),
        Keycode::R => LogicalKey::Character('r'),
        Keycode::S => LogicalKey::Character('s'),
        Keycode::T => LogicalKey::Character('t'),
        Keycode::U => LogicalKey::Character('u'),
        Keycode::V => LogicalKey::Character('v'),
        Keycode::W => LogicalKey::Character('w'),
        Keycode::X => LogicalKey::Character('x'),
        Keycode::Y => LogicalKey::Character('y'),
        Keycode::Z => LogicalKey::Character('z'),
        Keycode::Comma => LogicalKey::Character(','),
        Keycode::Period => LogicalKey::Character('.'),
        Keycode::AltLeft => LogicalKey::Named(NamedKey::Alt),
        Keycode::AltRight => LogicalKey::Named(NamedKey::Alt),
        Keycode::ShiftLeft => LogicalKey::Named(NamedKey::Shift),
        Keycode::ShiftRight => LogicalKey::Named(NamedKey::Shift),
        Keycode::Tab => LogicalKey::Named(NamedKey::Tab),
        Keycode::Space => LogicalKey::Character(' '),
        Keycode::Enter => LogicalKey::Named(NamedKey::Enter),
        Keycode::Del => LogicalKey::Named(NamedKey::Backspace),
        Keycode::Grave => LogicalKey::Character('`'),
        Keycode::Minus => LogicalKey::Character('-'),
        Keycode::Equals => LogicalKey::Character('='),
        Keycode::LeftBracket => LogicalKey::Character('['),
        Keycode::RightBracket => LogicalKey::Character(']'),
        Keycode::Backslash => LogicalKey::Character('\\'),
        Keycode::Semicolon => LogicalKey::Character(';'),
        Keycode::Apostrophe => LogicalKey::Character('\''),
        Keycode::Slash => LogicalKey::Character('/'),
        Keycode::Plus => LogicalKey::Character('+'),
        Keycode::PageUp => LogicalKey::Named(NamedKey::PageUp),
        Keycode::PageDown => LogicalKey::Named(NamedKey::PageDown),
        Keycode::Escape => LogicalKey::Named(NamedKey::Escape),
        Keycode::ForwardDel => LogicalKey::Named(NamedKey::Delete),
        Keycode::CtrlLeft => LogicalKey::Named(NamedKey::Control),
        Keycode::CtrlRight => LogicalKey::Named(NamedKey::Control),
        Keycode::CapsLock => LogicalKey::Named(NamedKey::CapsLock),
        Keycode::ScrollLock => LogicalKey::Named(NamedKey::ScrollLock),
        Keycode::Break => LogicalKey::Named(NamedKey::Pause),
        Keycode::MoveHome => LogicalKey::Named(NamedKey::Home),
        Keycode::MoveEnd => LogicalKey::Named(NamedKey::End),
        Keycode::Insert => LogicalKey::Named(NamedKey::Insert),
        Keycode::F1 => LogicalKey::Named(NamedKey::F1),
        Keycode::F2 => LogicalKey::Named(NamedKey::F2),
        Keycode::F3 => LogicalKey::Named(NamedKey::F3),
        Keycode::F4 => LogicalKey::Named(NamedKey::F4),
        Keycode::F5 => LogicalKey::Named(NamedKey::F5),
        Keycode::F6 => LogicalKey::Named(NamedKey::F6),
        Keycode::F7 => LogicalKey::Named(NamedKey::F7),
        Keycode::F8 => LogicalKey::Named(NamedKey::F8),
        Keycode::F9 => LogicalKey::Named(NamedKey::F9),
        Keycode::F10 => LogicalKey::Named(NamedKey::F10),
        Keycode::F11 => LogicalKey::Named(NamedKey::F11),
        Keycode::F12 => LogicalKey::Named(NamedKey::F12),
        Keycode::NumLock => LogicalKey::Named(NamedKey::NumLock),
        Keycode::Numpad0 => LogicalKey::Character('0'),
        Keycode::Numpad1 => LogicalKey::Character('1'),
        Keycode::Numpad2 => LogicalKey::Character('2'),
        Keycode::Numpad3 => LogicalKey::Character('3'),
        Keycode::Numpad4 => LogicalKey::Character('4'),
        Keycode::Numpad5 => LogicalKey::Character('5'),
        Keycode::Numpad6 => LogicalKey::Character('6'),
        Keycode::Numpad7 => LogicalKey::Character('7'),
        Keycode::Numpad8 => LogicalKey::Character('8'),
        Keycode::Numpad9 => LogicalKey::Character('9'),
        Keycode::NumpadDivide => LogicalKey::Character('/'),
        Keycode::NumpadMultiply => LogicalKey::Character('*'),
        Keycode::NumpadSubtract => LogicalKey::Character('-'),
        Keycode::NumpadAdd => LogicalKey::Character('+'),
        Keycode::NumpadDot => LogicalKey::Character('.'),
        Keycode::NumpadComma => LogicalKey::Character(','),
        Keycode::NumpadEnter => LogicalKey::Named(NamedKey::Enter),
        Keycode::NumpadEquals => LogicalKey::Character('='),
        _ => return None,
    };

    let key_location = match android.key_code() {
        Keycode::AltLeft | Keycode::ShiftLeft | Keycode::CtrlLeft | Keycode::MetaLeft => {
            KeyLocation::Left
        }

        Keycode::AltRight | Keycode::ShiftRight | Keycode::CtrlRight | Keycode::MetaRight => {
            KeyLocation::Right
        }

        Keycode::NumLock
        | Keycode::Numpad0
        | Keycode::Numpad1
        | Keycode::Numpad2
        | Keycode::Numpad3
        | Keycode::Numpad4
        | Keycode::Numpad5
        | Keycode::Numpad6
        | Keycode::Numpad7
        | Keycode::Numpad8
        | Keycode::Numpad9
        | Keycode::NumpadDivide
        | Keycode::NumpadMultiply
        | Keycode::NumpadSubtract
        | Keycode::NumpadAdd
        | Keycode::NumpadDot
        | Keycode::NumpadComma
        | Keycode::NumpadEnter
        | Keycode::NumpadEquals
        | Keycode::NumpadLeftParen
        | Keycode::NumpadRightParen => KeyLocation::Numpad,

        _ => KeyLocation::Standard,
    };

    Some(KeyDescriptor {
        physical_key,
        logical_key,
        key_location,
    })
}

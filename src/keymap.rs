use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EV_KEY::*;
use uinput::event::keyboard::Key;

pub fn map_key(key: EV_KEY) -> Option<Key> {
    match key {
        KEY_1 => Some(Key::F1),
        KEY_2 => Some(Key::F2),
        KEY_3 => Some(Key::F3),
        KEY_4 => Some(Key::F4),
        KEY_5 => Some(Key::F5),
        KEY_6 => Some(Key::F6),
        KEY_7 => Some(Key::F7),
        KEY_8 => Some(Key::F8),
        KEY_9 => Some(Key::F9),
        KEY_0 => Some(Key::F10),
        KEY_MINUS => Some(Key::F11),
        KEY_EQUAL => Some(Key::F12),
        _ => None
    }
}

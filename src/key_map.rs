use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EV_KEY::*;
use std::collections::HashMap;
use std::fs;
use toml;
use uinput::event::keyboard::Key;

#[derive(Copy, Clone)]
pub struct KeyMapper {
    keys: [Key; 12]
}

#[derive(Deserialize)]
struct Config {
    keys: HashMap<String, String>,
}

impl KeyMapper {
    pub fn default() -> KeyMapper {
        KeyMapper {
            keys: [Key::_1, Key::_2, Key::_3, Key::_4, Key::_5, Key::_6, Key::_7, Key::_8, Key::_9, Key::_0, Key::Minus, Key::Equal]
        }
    }

    pub fn read_from_file(path: &str) -> Result<KeyMapper, String> {
        let contents = read_file_contents(path)?;
        let config: Config = toml::from_str(contents.as_str())
            .map_err(|e| format!("{}", e))?;

        let mut key_mapper = KeyMapper::default();

        for (k, v) in config.keys {
            let key_num = k.parse::<usize>()
                .map(|i| i - 1)
                .map_err(|e| format!("{}", e))?;

            if key_num >= 12 {
                return Err(format!("Invalid key number: {}", key_num));
            }

            let key = get_key_enum(v)?;

            key_mapper.keys[key_num] = key;
        }

        Ok(key_mapper)
    }

    pub fn map_key(self, key: EV_KEY) -> Option<Key> {
        return key_index(key)
            .and_then(|i| self.keys.get(i))
            .map(|k| *k);
    }
}

fn read_file_contents(path: &str) -> Result<String, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("{}", e))?;

    return Ok(contents);
}

fn key_index(key: EV_KEY) -> Option<usize> {
    match key {
        KEY_1 => Some(0),
        KEY_2 => Some(1),
        KEY_3 => Some(2),
        KEY_4 => Some(3),
        KEY_5 => Some(4),
        KEY_6 => Some(5),
        KEY_7 => Some(6),
        KEY_8 => Some(7),
        KEY_9 => Some(8),
        KEY_0 => Some(9),
        KEY_MINUS => Some(10),
        KEY_EQUAL => Some(11),
        _ => None
    }
}

fn get_key_enum(key: String) -> Result<Key, String> {
    for variant in Key::iter_variants() {
        if format!("{:?}", variant).eq(key.as_str()) {
            return Ok(variant);
        }
    }

    Err(format!("No such key: {}", key))
}

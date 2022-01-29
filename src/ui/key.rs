#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifierFlag {
    Shift,
    Control,
    Alternate,
    Command,
    CapsLock,
    NumericPad
}

pub type KeyCode = sdl2::keyboard::Keycode;

pub struct Key {
    key_code: KeyCode,
    modifier_flags: Vec<ModifierFlag>
}

impl Key {
    pub fn new(key_code: KeyCode, modifier_flags: Vec<ModifierFlag>) -> Key {
        Key {
            key_code,
            modifier_flags
        }
    }

    pub fn key_code(&self) -> KeyCode {
        self.key_code
    }

    pub fn modifier_flags(&self) -> &Vec<ModifierFlag> {
        &self.modifier_flags
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.key_code == other.key_code && self.modifier_flags == other.modifier_flags
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("key_code", &self.key_code)
            .field("modifier_flags", &self.modifier_flags)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let key = Key::new(KeyCode::A, vec![ModifierFlag::Shift]);
        assert_eq!(key.key_code(), KeyCode::A);
        assert_eq!(key.modifier_flags(), &vec![ModifierFlag::Shift]);
    }

    #[test]
    fn test_key_eq() {
        let key1 = Key::new(KeyCode::A, vec![ModifierFlag::Shift]);
        let key2 = Key::new(KeyCode::A, vec![ModifierFlag::Shift]);
        assert_eq!(key1, key2);

        let key3 = Key::new(KeyCode::B, vec![ModifierFlag::Shift]);
        let key4 = Key::new(KeyCode::A, vec![ModifierFlag::Alternate]);

        assert_ne!(key1, key3);
        assert_ne!(key1, key4);
    }
}

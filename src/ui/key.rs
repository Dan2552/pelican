pub enum ModifierFlag {
    Shift,
    Control,
    Alternate,
    Command
}

pub struct Key {
    modifier_flags: Vec<ModifierFlag>,
    characters: String,
    characters_ignoring_modifiers: String
}

impl Key {
    pub fn new(modifier_flags: Vec<ModifierFlag>, characters: String, characters_ignoring_modifiers: String) -> Key {
        Key {
            modifier_flags,
            characters,
            characters_ignoring_modifiers
        }
    }

    pub fn modifier_flags(&self) -> &Vec<ModifierFlag> {
        &self.modifier_flags
    }

    pub fn characters(&self) -> &String {
        &self.characters
    }

    pub fn characters_ignoring_modifiers(&self) -> &String {
        &self.characters_ignoring_modifiers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_flags() {
        let key = Key::new(vec![ModifierFlag::Shift], "".to_string(), "".to_string());
        assert_eq!(key.modifier_flags().len(), 1);
    }
}

pub mod text_insertion;

/// A snapshot of a carat's state for tracking along with history.
pub(crate) struct CaratSnapshot {
    character_index: usize,
    selection: Option<std::ops::Range<usize>>
}

impl CaratSnapshot {
    pub(crate) fn new(character_index: usize, selection: Option<std::ops::Range<usize>>) -> Self {
        Self {
            character_index,
            selection
        }
    }

    pub(crate) fn character_index(&self) -> usize {
        self.character_index
    }

    pub(crate) fn selection(&self) -> &Option<std::ops::Range<usize>> {
        &self.selection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carat_snapshot() {
        let snapshot = CaratSnapshot::new(5, None);
        assert_eq!(snapshot.character_index(), 5);
        assert_eq!(snapshot.selection(), &None);

        let snapshot = CaratSnapshot::new(3, Some(2..5));
        assert_eq!(snapshot.character_index(), 3);
        assert_eq!(snapshot.selection(), &Some(2..5));
    }
}

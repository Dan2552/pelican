/// A snapshot of a carat's state for tracking along with history.
pub struct CaratSnapshot {
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

impl Clone for CaratSnapshot {
    fn clone(&self) -> Self {
        Self {
            character_index: self.character_index,
            selection: self.selection.clone()
        }
    }
}

impl PartialEq for CaratSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.character_index == other.character_index && self.selection == other.selection
    }
}

impl std::fmt::Debug for CaratSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CaratSnapshot {{ character_index: {}, selection: {:?} }}", self.character_index, self.selection)
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

    #[test]
    fn test_carat_snapshot_clone() {
        let snapshot = CaratSnapshot::new(5, None);
        let snapshot_clone = snapshot.clone();
        assert_eq!(snapshot, snapshot_clone);
    }
}

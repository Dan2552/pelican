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

    pub(crate) fn selection_intersects(&self, other: &CaratSnapshot) -> bool {
        if self.selection.is_none() {
            return false;
        }

        if other.selection.is_none() {
            return false;
        }

        let self_range = self.selection.as_ref().unwrap();
        let other_range = other.selection.as_ref().unwrap();

        self_range.start < other_range.end && other_range.start < self_range.end
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

    #[test]
    fn test_selection_intersects() {
        // 1 [     ]
        // 2 [     ]
        let snapshot = CaratSnapshot::new(5, None);
        let snapshot_other = CaratSnapshot::new(5, None);
        assert!(!snapshot.selection_intersects(&snapshot_other));

        // 1 [     ]
        // 2 *
        let snapshot = CaratSnapshot::new(5, None);
        let snapshot_other = CaratSnapshot::new(5, Some(2..5));
        assert!(!snapshot.selection_intersects(&snapshot_other));

        // 1 *
        // 2 [     ]
        let snapshot = CaratSnapshot::new(5, Some(2..5));
        let snapshot_other = CaratSnapshot::new(5, None);
        assert!(!snapshot.selection_intersects(&snapshot_other));

        // 1 [  ---]
        // 2 [  ---]
        let snapshot = CaratSnapshot::new(5, Some(2..5));
        let snapshot_other = CaratSnapshot::new(5, Some(2..5));
        assert!(snapshot.selection_intersects(&snapshot_other));

        // 1 [  ---]
        // 2 [   ---]
        let snapshot = CaratSnapshot::new(5, Some(2..5));
        let snapshot_other = CaratSnapshot::new(5, Some(3..6));
        assert!(snapshot.selection_intersects(&snapshot_other));

        // 1 [  ---]
        // 2 [ --- ]
        let snapshot = CaratSnapshot::new(5, Some(2..5));
        let snapshot_other = CaratSnapshot::new(5, Some(1..4));
        assert!(snapshot.selection_intersects(&snapshot_other));

        // 1 [--   ]
        // 2 [   --]
        let snapshot = CaratSnapshot::new(5, Some(0..2));
        let snapshot_other = CaratSnapshot::new(5, Some(3..5));
        assert!(!snapshot.selection_intersects(&snapshot_other));

        // 1 [--   ]
        // 2 [  ---]
        let snapshot = CaratSnapshot::new(5, Some(0..2));
        let snapshot_other = CaratSnapshot::new(5, Some(2..4));
        assert!(!snapshot.selection_intersects(&snapshot_other));
    }
}

use std::cmp::Ordering;

pub struct BinarySortInsertArray<T> {
  store: Vec<T>
}

impl<T> BinarySortInsertArray<T> {
    fn new() -> BinarySortInsertArray<T> {
        BinarySortInsertArray {
            store: Vec::new()
        }
    }

    fn push(&mut self, new_element: T, f: fn(&T, &T) -> Ordering) {
        let result = self.store.binary_search_by(|comparison| f(&new_element, comparison));
        match result {
            Ok(index) => {
                self.store.insert(index, new_element);
            }
            Err(index) => {
                self.store.insert(index, new_element);
            }
        }
    }

    fn delete(&mut self, element_to_delete: T, f: fn(&T, &T) -> Ordering) {
        let result = self.store.binary_search_by(|comparison| f(&element_to_delete, comparison));
        match result {
            Ok(index) => {
                self.store.remove(index);
            }
            Err(_) => {
                // no op
            }
        }
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.store.iter()
    }

    fn count(&self) -> usize {
        self.store.len()
    }

    fn contains(&self, element: T, f: fn(&T, &T) -> Ordering) -> bool {
        let result = self.store.binary_search_by(|comparison| f(&element, comparison));
        match result {
            Ok(_) => {
                true
            }
            Err(_) => {
                false
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push__it_inserts_in_order_of_the_comparison_value() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);

        let expect: Vec<u32> = vec!(1, 2, 3, 5, 8);
        assert_eq!(described_instance.store, expect);
    }

    #[test]
    fn delete__it_deletes_in_order_of_the_comparison_value() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);
        let f: fn(&u32, &u32) -> Ordering = |element_to_delete, comparison| comparison.cmp(element_to_delete);

        let expect: Vec<u32> = vec!(1, 2, 5, 8);

        described_instance.delete(3, f);
        assert_eq!(described_instance.store, expect);

        let expect: Vec<u32> = vec!(1, 2, 5);

        described_instance.delete(8, f);
        assert_eq!(described_instance.store, expect);

        described_instance.delete(100, f);

        assert_eq!(described_instance.store, expect);
    }

    #[test]
    fn test_iter() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);

        let expect: Vec<u32> = vec!(1, 2, 3, 5, 8);
        let mut iter = described_instance.iter();
        for i in expect {
            assert_eq!(iter.next().unwrap(), &i);
        }
    }

    #[test]
    fn test_count() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);

        assert_eq!(described_instance.count(), 5);
    }

    #[test]
    fn test_contains() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);

        assert!(described_instance.contains(1, f));
        assert!(described_instance.contains(2, f));
        assert!(described_instance.contains(3, f));
        assert!(described_instance.contains(5, f));
        assert!(described_instance.contains(8, f));
        assert!(!described_instance.contains(0, f));
        assert!(!described_instance.contains(4, f));
        assert!(!described_instance.contains(9, f));
    }

    #[test]
    fn test_is_empty() {
        let mut described_instance: BinarySortInsertArray<u32> = BinarySortInsertArray::new();
        let f: fn(&u32, &u32) -> Ordering = |new_element, comparison| comparison.cmp(new_element);
        described_instance.push(1, f);
        described_instance.push(8, f);
        described_instance.push(2, f);
        described_instance.push(5, f);
        described_instance.push(3, f);

        assert!(!described_instance.is_empty());

        described_instance.delete(1, f);
        assert!(!described_instance.is_empty());

        described_instance.delete(2, f);
        assert!(!described_instance.is_empty());

        described_instance.delete(3, f);
        assert!(!described_instance.is_empty());

        described_instance.delete(5, f);
        assert!(!described_instance.is_empty());

        described_instance.delete(8, f);
        assert!(described_instance.is_empty());
    }
}

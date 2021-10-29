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

    fn delete(&self, element: T) where T: std::fmt::Debug {

    }
}
// impl<T: std::fmt::Debug> std::fmt::Debug for BinarySortInsertArray<T> {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(fmt, "hello")
//     }
// }

// impl<T> std::fmt::Display for BinarySortInsertArray<T> where T: std::fmt::Debug {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, String::from(format!("{:?}", self.store)))
//     }
// }

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
}

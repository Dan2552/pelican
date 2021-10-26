pub struct BinarySortInsertArray<'a> {
  comparison: &'a (Ord + a),
  store: mut Vec
}

impl<'a>BinarySortInsertArray<'a'> {
    fn new(comparison: &'a Ord) -> BinarySortInsertArray<'a> {
       BinarySortInsertArray { comparison: comparison, store: Vec::new() }
    }

    fn push(new_element: &'a Ord) {
      store.binary_search(existing_element.comparison >= new_element.comparison) {
          Ok(pos) => store.insert(pos, new_element)
          Err(pos) =>store.push(new_element)
      }
    }

    fn delete(element_to_delete: &'a Ord) {
      let pos = store.binary_search(element_to_delete).unwrap_or_else(|e| e);
      store.remove(pos)
    }
}


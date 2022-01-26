/// Finds a word boundary in a string.
pub fn find_word_boundary(text: &str, index: usize, rightwards: bool) -> usize {
    let mut index = index;
    let mut has_seen_at_least_one_non_boundary = false;
    let mut only_whitespace_boundaries = true;

    if rightwards {
        while index < text.len() {
            let character = text.chars().nth(index).unwrap();
            if !(character.is_alphanumeric() || character == '_') {
                let whitespace = character.is_whitespace();

                if !whitespace { only_whitespace_boundaries = false; }

                // if we've seen anything that isn't whitespace before, but now
                // it's a whitespace, we end.
                if !only_whitespace_boundaries && whitespace {
                    break;
                }

                // if we've seen a non-boundary character before and now we've
                // seen a boundary, we end.
                if has_seen_at_least_one_non_boundary {
                    break;
                }
            } else {
                has_seen_at_least_one_non_boundary = true;
                only_whitespace_boundaries = false;
            }

            index += 1;
        }
    } else {
        while index > 0 {
            let character = text.chars().nth(index - 1).unwrap();
            if !(character.is_alphanumeric() || character == '_') {
                let whitespace = character.is_whitespace();

                if !whitespace { only_whitespace_boundaries = false; }

                // if we've seen anything that isn't whitespace before, but now
                // it's a whitespace, we end.
                if !only_whitespace_boundaries && whitespace {
                    break;
                }

                // if we've seen a non-boundary character before and now we've
                // seen a boundary, we end.
                if has_seen_at_least_one_non_boundary {
                    break;
                }
            } else {
                has_seen_at_least_one_non_boundary = true;
                only_whitespace_boundaries = false;
            }

            index -= 1;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_boundary() {
        assert_eq!(find_word_boundary("hello world", 0, true), 5);
        assert_eq!(find_word_boundary("hello world", 1, true), 5);
        assert_eq!(find_word_boundary("hello world", 2, true), 5);
        assert_eq!(find_word_boundary("hello world", 4, true), 5);
        assert_eq!(find_word_boundary("hello world", 5, true), 11);
        assert_eq!(find_word_boundary("hello world", 6, true), 11);
        assert_eq!(find_word_boundary("hello world", 7, true), 11);
        assert_eq!(find_word_boundary("hello world", 8, true), 11);
        assert_eq!(find_word_boundary("hello world", 9, true), 11);
        assert_eq!(find_word_boundary("hello world", 10, true), 11);
        assert_eq!(find_word_boundary("hello world", 11, true), 11);

        assert_eq!(find_word_boundary("hello world", 0, false), 0);
        assert_eq!(find_word_boundary("hello world", 1, false), 0);
        assert_eq!(find_word_boundary("hello world", 2, false), 0);
        assert_eq!(find_word_boundary("hello world", 4, false), 0);
        assert_eq!(find_word_boundary("hello world", 5, false), 0);
        assert_eq!(find_word_boundary("hello world", 6, false), 0);
        assert_eq!(find_word_boundary("hello world", 7, false), 6);
        assert_eq!(find_word_boundary("hello world", 8, false), 6);
        assert_eq!(find_word_boundary("hello world", 9, false), 6);
        assert_eq!(find_word_boundary("hello world", 10, false), 6);
        assert_eq!(find_word_boundary("hello world", 11, false), 6);

        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 0, true), 2);
        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 2, true), 21);
        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 21, true), 26);
        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 26, true), 27);
        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 27, true), 32);
        assert_eq!(find_word_boundary("fn find_word_boundary(text: &str,", 32, true), 33);
    }
}

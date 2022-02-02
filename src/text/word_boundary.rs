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

/// Finds a line boundary in a string.
pub fn find_line_boundary(text: &str, start_index: usize, rightwards: bool) -> usize {
    let mut index = start_index as i32;
    let mut vector = -1 as i32;
    if rightwards == true { vector = 1; }

    loop {
        let character = text.chars().nth(index as usize);

        if rightwards {
            if index >= text.len() as i32 {
                break;
            }

            if character.is_some() && character.unwrap() == '\n' {
                break;
            }
        } else {
            if index <= 0 {
                break;
            }

            if let Some(character) = text.chars().nth((index - 1) as usize) {
                if character == '\n' {
                    break;
                }
            }
        }

        index = index + vector;
    }

    index as usize
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

        assert_eq!(find_word_boundary("hello world hello world", 12, true), 17);
    }

    #[test]
    fn test_find_line_boundary() {
        let single_line = "a b c";

        assert_eq!(find_line_boundary(single_line, 0, true), 5);
        assert_eq!(find_line_boundary(single_line, 1, true), 5);
        assert_eq!(find_line_boundary(single_line, 2, true), 5);
        assert_eq!(find_line_boundary(single_line, 3, true), 5);
        assert_eq!(find_line_boundary(single_line, 4, true), 5);
        assert_eq!(find_line_boundary(single_line, 5, true), 5);

        assert_eq!(find_line_boundary(single_line, 0, false), 0);
        assert_eq!(find_line_boundary(single_line, 1, false), 0);
        assert_eq!(find_line_boundary(single_line, 2, false), 0);
        assert_eq!(find_line_boundary(single_line, 3, false), 0);
        assert_eq!(find_line_boundary(single_line, 4, false), 0);
        assert_eq!(find_line_boundary(single_line, 5, false), 0);

        let multiline = "a b\nc d\ne f";

        assert_eq!(find_line_boundary(multiline, 0, true), 3);
        assert_eq!(find_line_boundary(multiline, 1, true), 3);
        assert_eq!(find_line_boundary(multiline, 2, true), 3);
        assert_eq!(find_line_boundary(multiline, 3, true), 3);
        assert_eq!(find_line_boundary(multiline, 4, true), 7);
        assert_eq!(find_line_boundary(multiline, 5, true), 7);
        assert_eq!(find_line_boundary(multiline, 6, true), 7);
        assert_eq!(find_line_boundary(multiline, 7, true), 7);
        assert_eq!(find_line_boundary(multiline, 8, true), 11);
        assert_eq!(find_line_boundary(multiline, 9, true), 11);
        assert_eq!(find_line_boundary(multiline, 10, true), 11);
        assert_eq!(find_line_boundary(multiline, 11, true), 11);

        assert_eq!(find_line_boundary(multiline, 0, false), 0);
        assert_eq!(find_line_boundary(multiline, 1, false), 0);
        assert_eq!(find_line_boundary(multiline, 2, false), 0);
        assert_eq!(find_line_boundary(multiline, 3, false), 0);
        assert_eq!(find_line_boundary(multiline, 4, false), 4);
        assert_eq!(find_line_boundary(multiline, 5, false), 4);
        assert_eq!(find_line_boundary(multiline, 6, false), 4);
        assert_eq!(find_line_boundary(multiline, 7, false), 4);
        assert_eq!(find_line_boundary(multiline, 8, false), 8);
        assert_eq!(find_line_boundary(multiline, 9, false), 8);
        assert_eq!(find_line_boundary(multiline, 10, false), 8);
        assert_eq!(find_line_boundary(multiline, 11, false), 8);
    }
}

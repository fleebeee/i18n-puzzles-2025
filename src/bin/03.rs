i18n_puzzles::solution!(3);
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

fn has_valid_length(password: &str) -> bool {
    let len = password.graphemes(true).count();

    len >= 4 && len <= 12
}

fn has_at_least_one_digit(password: &str) -> bool {
    for grapheme in password.graphemes(true) {
        if grapheme.len() == 1 {
            let b = grapheme.as_bytes()[0];
            if b >= b'0' && b <= b'9' {
                return true;
            }
        }
    }

    false
}

// a length of at least 4 and at most 12
// at least one digit
// at least one uppercase letter (with or without accents, examples: A or Ż)
// at least one lowercase letter (with or without accents, examples: a or ŷ)
// at least one character that is outside the standard 7-bit ASCII character set (examples: Ű, ù or ř)

fn has_at_least_one_uppercase(password: &str) -> bool {
    for grapheme in password.graphemes(true) {
        let base_char = grapheme.nfd().next().unwrap();
        if base_char.is_ascii_uppercase() {
            return true;
        }
    }

    false
}

fn has_at_least_one_lowercase(password: &str) -> bool {
    for grapheme in password.graphemes(true) {
        let base_char = grapheme.nfd().next().unwrap();
        if base_char.is_ascii_lowercase() {
            return true;
        }
    }

    false
}

fn has_at_least_one_non_ascii(password: &str) -> bool {
    for grapheme in password.graphemes(true) {
        if grapheme.len() > 1 {
            return true;
        }

        if grapheme.as_bytes()[0] > 127 {
            return true;
        }
    }

    false
}

fn is_valid(password: &str) -> bool {
    // print!("{password}: ");

    // if !has_valid_length(password) {
    //     println!("invalid length");
    //     return false;
    // }

    // if !has_at_least_one_digit(password) {
    //     println!("no digits");
    //     return false;
    // }

    // if !has_at_least_one_uppercase(password) {
    //     println!("no uppercase letters");
    //     return false;
    // }

    // if !has_at_least_one_lowercase(password) {
    //     println!("no lowercase letters");
    //     return false;
    // }

    // if !has_at_least_one_non_ascii(password) {
    //     println!("no non-ascii letters");
    //     return false;
    // }

    // println!("valid");
    // true

    has_valid_length(password)
        && has_at_least_one_digit(password)
        && has_at_least_one_uppercase(password)
        && has_at_least_one_lowercase(password)
        && has_at_least_one_non_ascii(password)
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines().filter(|password| is_valid(password)).count();

    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

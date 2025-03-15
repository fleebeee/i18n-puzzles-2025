i18n_puzzles::solution!(8);

use hashbrown::HashSet;
use unicode_normalization::char::decompose_canonical;

fn has_valid_length(password: &[u8]) -> bool {
    let len = password.len();

    len >= 4 && len <= 12
}

fn has_at_least_one_digit(password: &[u8]) -> bool {
    for b in password {
        if *b >= b'0' && *b <= b'9' {
            return true;
        }
    }

    false
}

const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

fn has_at_least_one_vowel(password: &[u8]) -> bool {
    for b in password {
        if VOWELS.contains(&b.to_ascii_lowercase()) {
            return true;
        }
    }

    false
}

fn has_at_least_one_consonant(password: &[u8]) -> bool {
    for b in password {
        if b.is_ascii_alphabetic() && !VOWELS.contains(&b.to_ascii_lowercase()) {
            return true;
        }
    }

    false
}

fn no_recurring(password: &[u8]) -> bool {
    let mut seen = HashSet::new();

    for b in password {
        let c = b.to_ascii_lowercase();

        if seen.contains(&c) {
            return false;
        }

        seen.insert(c);
    }

    true
}

fn remove_accents(s: &str) -> String {
    let chars: String = s
        .chars()
        .map(|c| {
            let mut base_char = None;
            decompose_canonical(c, |c_new| {
                base_char.get_or_insert(c_new);
            });
            base_char.unwrap()
        })
        .collect();

    chars
}

fn is_valid(password: &str) -> bool {
    let unaccented_string = remove_accents(password);
    let password = unaccented_string.as_bytes();

    has_valid_length(password)
        && has_at_least_one_digit(password)
        && has_at_least_one_consonant(password)
        && has_at_least_one_vowel(password)
        && no_recurring(password)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

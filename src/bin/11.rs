use aho_corasick::AhoCorasick;

i18n_puzzles::solution!(11);

const ALPHABET_LEN: usize = 24;
const ALPHABET: &str = "αβγδεζηθικλμνξοπρστυφχψω";
const VARIANTS: [&str; 5] = ["οδυσσευς", "οδυσσεως", "οδυσσει", "οδυσσεα", "οδυσσευ"];

fn shift(s: String) -> String {
    let mut shifted = String::new();

    for c in s.chars() {
        let position = ALPHABET.chars().position(|b| b == c);

        if position.is_none() {
            shifted.push(c);
            continue;
        }

        // This could be done at compile time but eh
        let alphabet_vec: Vec<char> = ALPHABET.chars().collect();
        shifted.push(alphabet_vec[(position.unwrap() + 1) % ALPHABET_LEN]);
    }

    shifted
}

pub fn part_one(input: &str) -> Option<u64> {
    let ac = AhoCorasick::new(VARIANTS).unwrap();
    let mut total = 0;

    for line in input.lines() {
        let line = line.to_lowercase();
        let line = line.replace("ς", "σ");
        let mut current = line;

        for i in 0..ALPHABET_LEN {
            if ac.find(&current).is_some() {
                total += i;
                break;
            }

            current = shift(current);
        }
    }

    Some(total as u64)
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
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

i18n_puzzles::solution!(13);
use encoding::{Encoding, all::ISO_8859_1};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Clue {
    known: (usize, String),
    length: usize,
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Clue>) {
    let (words_str, crossword_str) = input.split_once("\n\n").unwrap();

    let words = words_str
        .lines()
        .map(|s| {
            // Remove BOMs
            if s.starts_with("feff") || s.starts_with("fffe") {
                return s[4..].to_string();
            } else if s.starts_with("efbbbf") {
                return s[6..].to_string();
            }

            s.to_string()
        })
        .collect();

    let mut clues = vec![];

    // All of the clues seem to be ASCII so we can get away with the following
    for clue in crossword_str.lines() {
        let clue = clue.trim();
        let length = clue.len();
        let bytes = clue.as_bytes();
        let position = bytes.iter().position(|c| *c != b'.').unwrap();
        let character = bytes[position] as char;

        clues.push(Clue {
            known: (position, character.to_string()),
            length,
        });
    }

    (words, clues)
}

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (words, clues) = parse_input(input);

    let mut total = 0;

    'words: for (i, word) in words.iter().enumerate() {
        let bytes = hex_to_bytes(&word);

        let utf8 = String::from_utf8(bytes.clone()).unwrap_or_default();

        let double_bytes_be: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| ((c[0] as u16) << 8) + c[1] as u16)
            .collect();
        let utf16_be = String::from_utf16(&double_bytes_be).unwrap_or_default();

        let double_bytes_le: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| ((c[1] as u16) << 8) + c[0] as u16)
            .collect();
        let utf16_le = String::from_utf16(&double_bytes_le).unwrap_or_default();

        let latin = ISO_8859_1
            .decode(&bytes, encoding::DecoderTrap::Strict)
            .unwrap_or_default();

        let candidates = [utf8, utf16_be, utf16_le, latin];

        for clue in &clues {
            for (j, candidate) in candidates.iter().enumerate() {
                // Latin decoding works even when we have weird results like Ã¶
                // luckily these can be filtered out by checking if we end up with uppercase
                // letters. This feels hacky but I don't know how else to solve this
                let has_uppercase = candidate.chars().filter(|c| c.is_uppercase()).count() > 0;
                if has_uppercase {
                    continue;
                }

                let graphemes: Vec<&str> = candidate.graphemes(true).collect();
                if graphemes.len() != clue.length {
                    continue;
                }

                if graphemes[clue.known.0] != clue.known.1 {
                    continue;
                }

                total += i + 1;
                // println!("Match: {}, {}", candidate, j);
                continue 'words;
            }
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
        assert_eq!(result, Some(47));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

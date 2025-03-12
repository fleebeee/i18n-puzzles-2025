i18n_puzzles::solution!(6);
use encoding::{Encoding, all::ISO_8859_1};
use std::str::from_utf8;

struct Clue {
    known: (usize, char),
    length: usize,
}

fn fix(word: String) -> String {
    let fixed = ISO_8859_1
        .encode(&word, encoding::EncoderTrap::Strict)
        .unwrap();

    from_utf8(&fixed).unwrap().to_string()
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Clue>) {
    let mut words = vec![];

    let (words_str, crossword_str) = input.split_once("\n\n").unwrap();

    for (i, word) in words_str.lines().enumerate() {
        let mut word = word.to_string();

        if (i + 1) % 3 == 0 {
            word = fix(word);
        }

        if (i + 1) % 5 == 0 {
            word = fix(word);
        }

        words.push(word)
    }

    let mut clues = vec![];

    // All of the clues seem to be ASCII so we can get away with the following
    for clue in crossword_str.lines() {
        let clue = clue.trim();
        let length = clue.len();
        let bytes = clue.as_bytes();
        let position = bytes.iter().position(|c| *c != b'.').unwrap();
        let character = bytes[position] as char;

        clues.push(Clue {
            known: (position, character),
            length,
        });
    }

    (words, clues)
}

fn matches(clue: &Clue, word: &String) -> bool {
    if word.chars().count() != clue.length {
        return false;
    }

    let c = word.chars().nth(clue.known.0).unwrap();

    clue.known.1 == c
}

pub fn part_one(input: &str) -> Option<usize> {
    let (words, clues) = parse_input(input);

    let mut total = 0;

    for clue in &clues {
        for (i, word) in words.iter().enumerate() {
            if matches(clue, word) {
                total += i + 1;
                break;
            }
        }
    }

    Some(total)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

i18n_puzzles::solution!(12);

#[derive(Debug)]
struct Contact {
    first: String,
    last: String,
    number: u64,
}

fn parse_input(input: &str) -> Vec<Contact> {
    let mut contacts = vec![];

    for line in input.lines() {
        let (last, rest) = line.split_once(", ").unwrap();
        let (first, number) = rest.split_once(": ").unwrap();

        contacts.push(Contact {
            first: first.to_string(),
            last: last.to_string(),
            number: number.parse().unwrap(),
        });
    }

    contacts
}

fn apply_english(word: &String) -> String {
    let word = word.to_lowercase();
    let word = word.replace("æ", "ae");
    let word = word.replace("ı", "i");
    let word = diacritics::remove_diacritics(&word);

    let word: String = word
        .chars()
        .map(|x| match x {
            '\'' => "".to_string(),
            ' ' => "".to_string(),
            _ => x.to_string(),
        })
        .collect();

    word
}

fn swedish_alphabetical_order(c: char) -> u8 {
    match c.to_lowercase().next().unwrap_or(c) {
        x @ 'a'..='z' => x as u8 - b'a' + 1,
        'å' => 27,
        'ä' => 28,
        'ö' => 29,
        _ => 30, // Any other characters go at the end
    }
}

fn apply_swedish(word: &String) -> String {
    let word = word.to_lowercase();
    let word = word.replace("æ", "ä");
    let word = word.replace("ø", "ö");
    let word = word.replace("ı", "i");

    // Preserve only åäö
    let word: String = word
        .nfc()
        .to_string()
        .graphemes(false)
        .map(|x| {
            if x.len() > 1 {
                let base_char = x.nfd().next().unwrap();
                return match base_char {
                    'a' | 'o' => x.to_string(),
                    _ => base_char.to_string(),
                };
            }

            match x {
                "\'" => "".to_string(),
                " " => "".to_string(),
                "-" => "".to_string(),
                _ => x.to_string(),
            }
        })
        .collect();

    word
}

fn swedish_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let a_chars = a.chars().map(swedish_alphabetical_order);
    let b_chars = b.chars().map(swedish_alphabetical_order);
    a_chars.cmp(b_chars)
}

fn apply_dutch(word: &String) -> String {
    let word = diacritics::remove_diacritics(&word);
    let first_capital = word.chars().position(|c| c.is_ascii_uppercase()).unwrap();
    let word = word[first_capital..].to_string();

    let word = word.to_lowercase();
    let word = word.replace("æ", "ae");
    let word = word.replace("ı", "i");

    let word: String = word
        .chars()
        .map(|x| match x {
            '\'' => "".to_string(),
            ' ' => "".to_string(),
            _ => x.to_string(),
        })
        .collect();

    word
}

pub fn part_one(input: &str) -> Option<u64> {
    let contacts = parse_input(input);

    let mut english_contacts: Vec<Contact> = contacts
        .iter()
        .map(|c| Contact {
            first: apply_english(&c.first),
            last: apply_english(&c.last),
            number: c.number,
        })
        .collect();

    english_contacts.sort_unstable_by(|a, b| a.last.cmp(&b.last).then(a.first.cmp(&b.first)));

    let english_result = english_contacts[english_contacts.len() / 2].number;

    let mut swedish_contacts: Vec<Contact> = contacts
        .iter()
        .map(|c| Contact {
            first: apply_swedish(&c.first),
            last: apply_swedish(&c.last),
            number: c.number,
        })
        .collect();

    swedish_contacts.sort_unstable_by(|a, b| {
        swedish_cmp(&a.last, &b.last).then(swedish_cmp(&a.first, &b.first))
    });

    let swedish_result = swedish_contacts[swedish_contacts.len() / 2].number;

    let mut dutch_contacts: Vec<Contact> = contacts
        .iter()
        .map(|c| Contact {
            first: apply_dutch(&c.first),
            last: apply_dutch(&c.last),
            number: c.number,
        })
        .collect();

    dutch_contacts.sort_unstable_by(|a, b| a.last.cmp(&b.last).then(a.first.cmp(&b.first)));

    let dutch_result = dutch_contacts[dutch_contacts.len() / 2].number;

    let result = english_result * swedish_result * dutch_result;

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
        assert_eq!(result, Some(1885816494308838));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use hashbrown::HashMap;
use rayon::prelude::*;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

i18n_puzzles::solution!(10);

#[derive(Debug)]
struct Entry {
    username: String,
    hash: String,
}

#[derive(Debug)]
struct Attempt {
    username: String,
    password: String,
}

fn parse_input(input: &str) -> (Vec<Entry>, Vec<Attempt>) {
    let mut entries = vec![];
    let mut attempts = vec![];

    let (entries_str, attempts_str) = input.split_once("\n\n").unwrap();

    for line in entries_str.lines() {
        let (username, hash) = line.split_once(' ').unwrap();

        entries.push(Entry {
            username: username.to_string(),
            hash: hash.to_string(),
        });
    }

    for line in attempts_str.lines() {
        let (username, password) = line.split_once(' ').unwrap();

        attempts.push(Attempt {
            username: username.to_string(),
            password: password.to_string(),
        });
    }

    (entries, attempts)
}

fn has_accent(c: &str) -> bool {
    let mut decomposed = c.nfd();
    let _base_char = decomposed.next();
    decomposed.next().is_some() // If there's another character after the base, it's an accent
}

fn get_variations(input: &str, mut base: String) -> Vec<String> {
    if input.is_empty() {
        return vec![base];
    }

    let mut i = 0;
    for c in input.graphemes(false) {
        if has_accent(c) {
            // println!("Splitting at {c}");
            let mut variations = vec![];
            variations.extend(get_variations(&input[i + c.len()..], base.clone() + c));
            variations.extend(get_variations(
                &input[i + c.len()..],
                base + &c.nfd().to_string(),
            ));
            return variations;
        }

        i += c.len();
        base.push_str(c);
    }

    vec![base]
}

pub fn part_one(input: &str) -> Option<u64> {
    let (entries, attempts) = parse_input(input);

    let cracked_passwords = std::sync::Mutex::new(HashMap::new());

    let total = attempts
        .par_iter()
        .filter(|Attempt { username, password }| {
            let entry = entries.iter().find(|e| e.username == *username).unwrap();

            // Try every variation of password with expanded/compact characters
            let normalized_password = password.nfc().to_string();

            if let Some(cracked) = cracked_passwords.lock().unwrap().get(&username) {
                if normalized_password == *cracked {
                    return true;
                }
            }

            let variations = get_variations(&normalized_password, String::new());

            for variation in variations {
                let hash = bcrypt::verify(&variation, &entry.hash).unwrap();

                if hash {
                    // Memorize cracked passwords
                    cracked_passwords
                        .lock()
                        .unwrap()
                        .insert(username, normalized_password);
                    return true;
                }
            }

            false
        })
        .count();

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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

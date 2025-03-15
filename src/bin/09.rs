i18n_puzzles::solution!(9);
use chrono::{NaiveDateTime, format::ParseErrorKind};
use hashbrown::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut authors_map = HashMap::new();

    for line in input.lines() {
        let (date, authors) = line.split_once(": ").unwrap();
        for author in authors.split(", ") {
            authors_map
                .entry(author.to_string())
                .or_insert(vec![])
                .push(date.to_string())
        }
    }

    authors_map
}

#[derive(PartialEq)]
enum Field {
    Day,
    Month,
    Year,
}

type Format = [Field; 3];

const DATE_FORMATS: [Format; 6] = [
    [Field::Day, Field::Month, Field::Year],
    [Field::Day, Field::Year, Field::Month],
    [Field::Month, Field::Day, Field::Year],
    [Field::Month, Field::Year, Field::Day],
    [Field::Year, Field::Day, Field::Month],
    [Field::Year, Field::Month, Field::Day],
];

fn format_to_string(format: &Format) -> String {
    format
        .iter()
        .map(|field| match field {
            Field::Day => "%d",
            Field::Month => "%m",
            Field::Year => "%y",
        })
        .collect::<Vec<&str>>()
        .join("-")
}

fn get_indices(format: &Format) -> (usize, usize, usize) {
    let year = format.iter().position(|f| *f == Field::Year).unwrap();
    let month = format.iter().position(|f| *f == Field::Month).unwrap();
    let day = format.iter().position(|f| *f == Field::Day).unwrap();

    (year, month, day)
}

fn is_viable_format(format: &Format, dates: &[String]) -> bool {
    let format_str = format_to_string(format);

    for date_str in dates {
        let naive = NaiveDateTime::parse_from_str(date_str, &format_str);

        match naive {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ParseErrorKind::NotEnough => (), // We count this as a success
                _ => return false,
            },
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<String> {
    let authors = parse_input(input);
    let mut result = vec![];

    for (author, dates) in authors {
        let date_format = DATE_FORMATS.iter().find(|f| is_viable_format(f, &dates))?;
        let (year, month, day) = get_indices(date_format);

        for date in dates {
            let date: Vec<_> = date.split('-').collect();
            if date[year] == "01" && date[month] == "09" && date[day] == "11" {
                result.push(author.clone());
                break;
            }
        }
    }

    result.sort_unstable();

    Some(result.join(" "))
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

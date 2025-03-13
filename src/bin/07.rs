i18n_puzzles::solution!(7);
use chrono::{DateTime, FixedOffset, Offset, TimeZone, Timelike};
use chrono_tz::Tz;
use itertools::Itertools;
use std::str::FromStr;

fn determine_iana_from_options(date: DateTime<FixedOffset>, options: &[String]) -> Option<String> {
    let offset = date.offset().fix();
    let naive = date.naive_utc();

    for option in options {
        let timezone = Tz::from_str(option).unwrap();
        let guess = timezone.from_utc_datetime(&naive).offset().fix();

        if guess == offset {
            return Some(option.clone());
        }
    }

    None
}

fn fix_date(line: &str) -> DateTime<Tz> {
    let (date_str, right, wrong) = line.split('\t').collect_tuple().unwrap();

    let date = DateTime::parse_from_rfc3339(date_str).unwrap();
    let wrong = wrong.parse::<i64>().unwrap() * 60;
    let right = right.parse::<i64>().unwrap() * 60;

    let iana = determine_iana_from_options(
        date,
        &[
            "America/Halifax".to_string(),
            "America/Santiago".to_string(),
        ],
    )
    .unwrap();

    let fixed_timestamp = date.timestamp() - wrong + right;
    let utc = DateTime::from_timestamp(fixed_timestamp, 0).unwrap();
    let naive = utc.naive_utc();

    let timezone = Tz::from_str(&iana).unwrap();
    let fixed_date = timezone.from_utc_datetime(&naive);

    fixed_date
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let fixed_date = fix_date(line);

            (i + 1) as u32 * fixed_date.hour()
        })
        .sum();

    Some(result)
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
        assert_eq!(result, Some(866));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

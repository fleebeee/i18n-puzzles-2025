i18n_puzzles::solution!(2);
use hashbrown::HashMap;
use time::format_description::well_known::Iso8601;
use time::{OffsetDateTime, format_description};

fn find_timestamp(input: &str) -> Option<i64> {
    let mut timestamps: HashMap<i64, u8> = HashMap::new();

    for line in input.lines() {
        let date = OffsetDateTime::parse(line, &Iso8601::DEFAULT).unwrap();
        let timestamp = date.unix_timestamp();
        let entry = timestamps.entry(timestamp);
        *entry.or_default() += 1;

        if timestamps[&timestamp] == 4 {
            return Some(timestamp);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<String> {
    let timestamp = find_timestamp(input)?;

    let result = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();

    let format = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour \
             sign:mandatory]:[offset_minute]",
    )
    .unwrap();

    let iso8601_string = result.format(&format).unwrap();

    Some(iso8601_string)
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
        assert_eq!(result, Some(String::from("2019-06-05T12:15:00+00:00")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

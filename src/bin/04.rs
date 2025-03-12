i18n_puzzles::solution!(4);
use i18n_puzzles::utils::time::parse_date_with_iana;
use regex::Regex;

// code mostly stolen from bramhaag
const DATE_FORMAT: &str = "%b %d, %Y, %H:%M";

pub fn part_one(input: &str) -> Option<i64> {
    let pattern = Regex::new(r"Departure:\s+(.*?)\s+(.*)\nArrival:\s+(.*?)\s+(.*)\n").unwrap();

    let total_minutes = pattern
        .captures_iter(&input)
        .map(|cap| {
            let dep = parse_date_with_iana(&cap[2], &cap[1], DATE_FORMAT);
            let arr = parse_date_with_iana(&cap[4], &cap[3], DATE_FORMAT);
            (arr.timestamp() - dep.timestamp()) / 60
        })
        .sum();

    Some(total_minutes)
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

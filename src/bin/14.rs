i18n_puzzles::solution!(14);
use suukon::japanese::kanji_to_num;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Distance {
    numerator: u64,
    denumerator: u64,
}

fn get_dec_unit(unit: &str) -> (u64, u64) {
    let mut numerator = 10;
    let mut denumerator = 33;

    match unit {
        "尺" => (),
        "間" => numerator *= 6,
        "丈" => numerator *= 10,
        "町" => numerator *= 360,
        "里" => numerator *= 12_960,
        "毛" => denumerator *= 10_000,
        "厘" => denumerator *= 1_000,
        "分" => denumerator *= 100,
        "寸" => denumerator *= 10,
        _ => unreachable!(),
    }

    (numerator, denumerator)
}

fn parse_input(input: &str) -> Vec<(Distance, Distance)> {
    let mut expressions = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once(" × ").unwrap();
        let left_unit = left.graphemes(true).last().unwrap();
        let right_unit = right.graphemes(true).last().unwrap();
        let left_value = left
            .graphemes(true)
            .take(left.graphemes(true).count() - 1)
            .collect();
        let right_value = right
            .graphemes(true)
            .take(right.graphemes(true).count() - 1)
            .collect();
        let left_value: u64 = kanji_to_num(left_value, None).unwrap().parse().unwrap();
        let right_value: u64 = kanji_to_num(right_value, None).unwrap().parse().unwrap();

        // Apply unit
        let left_fraction = get_dec_unit(left_unit);
        let right_fraction = get_dec_unit(right_unit);

        expressions.push((
            Distance {
                numerator: left_fraction.0 * left_value,
                denumerator: left_fraction.1,
            },
            Distance {
                numerator: right_fraction.0 * right_value,
                denumerator: right_fraction.1,
            },
        ))
    }

    expressions
}

pub fn part_one(input: &str) -> Option<u64> {
    let expressions = parse_input(input);

    let mut total = 0;

    for (left, right) in expressions {
        let numerator = left.numerator * right.numerator;
        let denumerator = left.denumerator * right.denumerator;

        total += numerator / denumerator;
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
        assert_eq!(result, Some(2177741195));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

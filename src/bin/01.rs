i18n_puzzles::solution!(1);
use unicode_segmentation::UnicodeSegmentation;

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .into_iter()
        .map(|line| {
            let bytes = line.as_bytes().len();
            let characters = line.graphemes(true).count();

            if bytes <= 160 {
                if characters <= 140 { 13 } else { 11 }
            } else if characters <= 140 {
                7
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
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

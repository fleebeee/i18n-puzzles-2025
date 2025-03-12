use i18n_puzzles::utils::{grid2d::Grid2D, vector2d::Vector2D};

i18n_puzzles::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid2D::parse(input);
    let mut current = Vector2D::new(0, 0);
    let bounds = Vector2D::new(grid.width, grid.height * 2);
    let direction = Vector2D::new(2, 1);
    let mut total = 0;

    while current.y < grid.height {
        if grid[current] == '💩' {
            total += 1;
        }

        current = (current + direction).wrap(&bounds);
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

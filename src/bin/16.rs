use codepage_437::{CP437_CONTROL, FromCp437};
use hashbrown::HashSet;
use i18n_puzzles::utils::{
    grid2d::Grid2D,
    vector2d::{DOWN, LEFT, RIGHT, UP, Vector2D},
};
use std::{collections::VecDeque, env, fs};

i18n_puzzles::solution!(16);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Connection {
    Single,
    Double,
}

fn index_to_dir(index: usize) -> Vector2D {
    match index {
        0 => RIGHT,
        1 => DOWN,
        2 => LEFT,
        3 => UP,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pipe {
    connections: [Option<Connection>; 4],
    locked: bool,
}

impl Pipe {
    fn rotate_cw(self: &mut Self) {
        self.connections.rotate_right(1);
    }
}

fn get_connections(c: char) -> [Option<Connection>; 4] {
    match c {
        '└' => [
            Some(Connection::Single),
            None,
            None,
            Some(Connection::Single),
        ],
        '┘' => [
            None,
            None,
            Some(Connection::Single),
            Some(Connection::Single),
        ],
        '─' => [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            None,
        ],
        '┐' => [
            None,
            Some(Connection::Single),
            Some(Connection::Single),
            None,
        ],
        '┌' => [
            Some(Connection::Single),
            Some(Connection::Single),
            None,
            None,
        ],
        '┬' => [
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
            None,
        ],
        '┤' => [
            None,
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
        ],
        '┴' => [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            Some(Connection::Single),
        ],
        '├' => [
            Some(Connection::Single),
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ],
        '┼' => [
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
        ],
        '│' => [
            None,
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ],
        '═' => [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            None,
        ],
        '║' => [
            None,
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ],
        '╒' => [
            Some(Connection::Double),
            Some(Connection::Single),
            None,
            None,
        ],
        '╓' => [
            Some(Connection::Single),
            Some(Connection::Double),
            None,
            None,
        ],
        '╔' => [
            Some(Connection::Double),
            Some(Connection::Double),
            None,
            None,
        ],
        '╕' => [
            None,
            Some(Connection::Single),
            Some(Connection::Double),
            None,
        ],
        '╖' => [
            None,
            Some(Connection::Double),
            Some(Connection::Single),
            None,
        ],
        '╗' => [
            None,
            Some(Connection::Double),
            Some(Connection::Double),
            None,
        ],
        '╘' => [
            Some(Connection::Double),
            None,
            None,
            Some(Connection::Single),
        ],
        '╙' => [
            Some(Connection::Single),
            None,
            None,
            Some(Connection::Double),
        ],
        '╚' => [
            Some(Connection::Double),
            None,
            None,
            Some(Connection::Double),
        ],
        '╛' => [
            None,
            None,
            Some(Connection::Double),
            Some(Connection::Single),
        ],
        '╜' => [
            None,
            None,
            Some(Connection::Single),
            Some(Connection::Double),
        ],
        '╝' => [
            None,
            None,
            Some(Connection::Double),
            Some(Connection::Double),
        ],
        '╞' => [
            Some(Connection::Double),
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ],
        '╟' => [
            Some(Connection::Single),
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ],
        '╠' => [
            Some(Connection::Double),
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ],
        '╡' => [
            None,
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
        ],
        '╢' => [
            None,
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
        ],
        '╣' => [
            None,
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
        ],
        '╤' => [
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
            None,
        ],
        '╥' => [
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
            None,
        ],
        '╦' => [
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
            None,
        ],
        '╧' => [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            Some(Connection::Single),
        ],
        '╨' => [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            Some(Connection::Double),
        ],
        '╩' => [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            Some(Connection::Double),
        ],
        '╪' => [
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
        ],
        '╫' => [
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
        ],
        '╬' => [
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
        ],
        _ => [None, None, None, None],
    }
}

fn print_grid(grid: &Grid2D<Option<Pipe>>) {
    for y in 0..grid.height {
        let mut line = String::new();
        for x in 0..grid.width {
            let pos = y * grid.width + x;
            if let Some(pipe) = &grid.bytes[pos as usize] {
                line.push(connections_to_char(&pipe.connections));
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

fn connections_to_char(connections: &[Option<Connection>; 4]) -> char {
    match connections {
        [
            Some(Connection::Single),
            None,
            None,
            Some(Connection::Single),
        ] => '└',
        [
            None,
            None,
            Some(Connection::Single),
            Some(Connection::Single),
        ] => '┘',
        [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            None,
        ] => '─',
        [
            None,
            Some(Connection::Single),
            Some(Connection::Single),
            None,
        ] => '┐',
        [
            Some(Connection::Single),
            Some(Connection::Single),
            None,
            None,
        ] => '┌',
        [
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
            None,
        ] => '┬',
        [
            None,
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
        ] => '┤',
        [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            Some(Connection::Single),
        ] => '┴',
        [
            Some(Connection::Single),
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ] => '├',
        [
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
            Some(Connection::Single),
        ] => '┼',
        [
            None,
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ] => '│',
        [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            None,
        ] => '═',
        [
            None,
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ] => '║',
        [
            Some(Connection::Double),
            Some(Connection::Single),
            None,
            None,
        ] => '╒',
        [
            Some(Connection::Single),
            Some(Connection::Double),
            None,
            None,
        ] => '╓',
        [
            Some(Connection::Double),
            Some(Connection::Double),
            None,
            None,
        ] => '╔',
        [
            None,
            Some(Connection::Single),
            Some(Connection::Double),
            None,
        ] => '╕',
        [
            None,
            Some(Connection::Double),
            Some(Connection::Single),
            None,
        ] => '╖',
        [
            None,
            Some(Connection::Double),
            Some(Connection::Double),
            None,
        ] => '╗',
        [
            Some(Connection::Double),
            None,
            None,
            Some(Connection::Single),
        ] => '╘',
        [
            Some(Connection::Single),
            None,
            None,
            Some(Connection::Double),
        ] => '╙',
        [
            Some(Connection::Double),
            None,
            None,
            Some(Connection::Double),
        ] => '╚',
        [
            None,
            None,
            Some(Connection::Double),
            Some(Connection::Single),
        ] => '╛',
        [
            None,
            None,
            Some(Connection::Single),
            Some(Connection::Double),
        ] => '╜',
        [
            None,
            None,
            Some(Connection::Double),
            Some(Connection::Double),
        ] => '╝',
        [
            Some(Connection::Double),
            Some(Connection::Single),
            None,
            Some(Connection::Single),
        ] => '╞',
        [
            Some(Connection::Single),
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ] => '╟',
        [
            Some(Connection::Double),
            Some(Connection::Double),
            None,
            Some(Connection::Double),
        ] => '╠',
        [
            None,
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
        ] => '╡',
        [
            None,
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
        ] => '╢',
        [
            None,
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
        ] => '╣',
        [
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
            None,
        ] => '╤',
        [
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
            None,
        ] => '╥',
        [
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
            None,
        ] => '╦',
        [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            Some(Connection::Single),
        ] => '╧',
        [
            Some(Connection::Single),
            None,
            Some(Connection::Single),
            Some(Connection::Double),
        ] => '╨',
        [
            Some(Connection::Double),
            None,
            Some(Connection::Double),
            Some(Connection::Double),
        ] => '╩',
        [
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
        ] => '╪',
        [
            Some(Connection::Single),
            Some(Connection::Double),
            Some(Connection::Single),
            Some(Connection::Double),
        ] => '╫',
        [
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
            Some(Connection::Double),
        ] => '╬',
        _ => ' ',
    }
}

fn parse_string(string: &String) -> Grid2D<Option<Pipe>> {
    let mut pipes = vec![];

    let lines: Vec<&str> = string.lines().collect();

    for line in &lines {
        for c in line.chars() {
            let connections = get_connections(c);
            if connections.iter().any(|c| c.is_some()) {
                pipes.push(Some(Pipe {
                    connections,
                    locked: false,
                }));
            } else {
                pipes.push(None);
            }
        }
    }

    Grid2D {
        width: lines[0].chars().count() as i32,
        height: lines.len() as i32,
        bytes: pipes,
    }
}

fn is_valid_pipe(grid: &Grid2D<Option<Pipe>>, point: Vector2D, strict: bool) -> bool {
    let pipe = grid[point];
    if pipe.is_none() {
        return true;
    }
    let pipe = pipe.unwrap();

    for (i, connection) in pipe.connections.iter().enumerate() {
        if connection.is_none() {
            continue;
        }
        let connection = connection.unwrap();
        let dir = index_to_dir(i);
        let neighbor_point = point + dir;

        // Pipe can't point at OOB
        if neighbor_point.x < 0
            || neighbor_point.x >= grid.width
            || neighbor_point.y < 0
            || neighbor_point.y >= grid.height
        {
            return false;
        }

        let neighbor = grid[neighbor_point];

        // Pipe can't point at empty space
        if neighbor.is_none() {
            return false;
        }

        if strict {
            let neighbor = neighbor.unwrap();

            let opposite = match i {
                0 => 2,
                1 => 3,
                2 => 0,
                3 => 1,
                _ => unreachable!(),
            };

            let opposite_connection = neighbor.connections[opposite];

            // Neighbor must have an opposing connection
            if opposite_connection.is_none() {
                return false;
            }

            let opposite_connection = opposite_connection.unwrap();

            // Single into single, double into double
            if opposite_connection != connection {
                return false;
            }
        }
    }

    true
}

fn is_solved(grid: &Grid2D<Option<Pipe>>, source: Vector2D, destination: Vector2D) -> bool {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Vector2D::new(x, y);

            // These points can point wherever
            if point == source || point == destination {
                continue;
            }

            if !is_valid_pipe(grid, point, true) {
                return false;
            }
        }
    }

    true
}

fn lock_grid(grid: &mut Grid2D<Option<Pipe>>) -> u32 {
    let mut rotations = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Vector2D::new(x, y);

            let pipe = grid[point];
            if pipe.is_none() {
                continue;
            }

            let mut pipe = pipe.unwrap();

            if pipe.locked {
                continue;
            }

            let mut possible_orientations = HashSet::new();
            let mut rotated_pipe = pipe.clone();

            'outer: for _ in 0..4 {
                rotated_pipe.rotate_cw();

                // Check each connection
                for (i, connection) in rotated_pipe.connections.iter().enumerate() {
                    if connection.is_none() {
                        continue;
                    }

                    let connection = connection.unwrap();

                    let dir = index_to_dir(i);
                    let neighbor_point = point + dir;

                    // If neighbor is out of bounds, we can't match
                    if neighbor_point.x < 0
                        || neighbor_point.x >= grid.width
                        || neighbor_point.y < 0
                        || neighbor_point.y >= grid.height
                    {
                        continue 'outer;
                    }

                    let neighbor = grid[neighbor_point];

                    // If there's no neighbor pipe, we can't match
                    if neighbor.is_none() {
                        continue 'outer;
                    }

                    let neighbor = neighbor.unwrap();

                    if neighbor.locked {
                        // If the neighbor is locked, the match must be exact
                        let opposite = match i {
                            0 => 2,
                            1 => 3,
                            2 => 0,
                            3 => 1,
                            _ => unreachable!(),
                        };

                        let other = neighbor.connections[opposite];

                        if other.is_none() {
                            continue 'outer;
                        }

                        let other = other.unwrap();

                        if other != connection {
                            continue 'outer;
                        }
                    } else {
                        // Check if any rotation of the neighbor
                        // can be a match
                        if !neighbor.connections.iter().any(|c| {
                            if c.is_none() {
                                return false;
                            }

                            let c = c.unwrap();

                            if c == connection {
                                return true;
                            }

                            false
                        }) {
                            continue 'outer;
                        }
                    }
                }

                possible_orientations.insert(rotated_pipe);
            }

            // dbg!(&possible_orientations.len());

            if possible_orientations.len() == 1 {
                // println!("Locking {point}");
                let mut new_pipe = possible_orientations.into_iter().next().unwrap();
                new_pipe.locked = true;
                grid[point] = Some(new_pipe);

                while pipe.connections != new_pipe.connections {
                    rotations += 1;
                    pipe.rotate_cw();
                }
            }
        }
    }

    rotations
}

fn bfs(grid: &Grid2D<Option<Pipe>>, source: Vector2D, destination: Vector2D) -> u32 {
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back((grid.clone(), 0));

    while let Some((current, rotations)) = frontier.pop_front() {
        // println!("rotations: {rotations}");
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        if is_solved(&current, source, destination) {
            println!("Solved grid");
            print_grid(&current);
            return rotations;
        }

        for y in 0..current.height {
            for x in 0..current.width {
                let point = Vector2D::new(x, y);

                let pipe = current[point];

                if pipe.is_none() {
                    continue;
                }

                let mut pipe = pipe.unwrap();
                if pipe.locked {
                    continue;
                }

                pipe.locked = true;

                for i in 0..4 {
                    let mut new_grid = current.clone();
                    new_grid[point] = Some(pipe);

                    let rots = lock_grid(&mut new_grid);

                    if is_valid_pipe(&new_grid, point, false) {
                        frontier.push_back((new_grid, rotations + i + rots));
                    }

                    pipe.rotate_cw();
                }
            }
        }
    }

    0
}

fn remove_frame(s: String) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let trimmed_lines = &lines[3..lines.len() - 4]; // Skip 3 lines from top and bottom
    let width_to_trim = 7;

    trimmed_lines
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() <= width_to_trim * 2 {
                "".to_string()
            } else {
                chars[width_to_trim..chars.len() - width_to_trim]
                    .iter()
                    .collect()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part_one(_input: &str) -> Option<u64> {
    let real = true;

    let cwd = env::current_dir().unwrap();
    let filepath = if real {
        cwd.join("data")
            .join("inputs")
            .join(format!("{DAY}_real.txt"))
    } else {
        cwd.join("data").join("examples").join(format!("{DAY}.txt"))
    };
    let bytes = fs::read(&filepath).expect("Failed to read file");
    let mut string = String::from_cp437(bytes, &CP437_CONTROL);

    if real {
        string = remove_frame(string);
    }

    let mut grid = parse_string(&string);

    print_grid(&grid);

    let source = Vector2D::new(0, 0);
    let destination = Vector2D::new((grid.width - 1) as i32, (grid.height - 1) as i32);

    // Lock source and destination
    let source_pipe = grid.bytes[0].as_mut().unwrap();
    source_pipe.locked = true;
    let destination_pipe = grid.bytes[(grid.width * grid.height - 1) as usize]
        .as_mut()
        .unwrap();
    destination_pipe.locked = true;

    let mut total_rotations = 0;
    loop {
        let rotations = lock_grid(&mut grid);

        if rotations == 0 {
            break;
        }

        total_rotations += rotations;
    }
    println!("After locking");

    print_grid(&grid);

    // let unlocked_count = grid
    //     .bytes
    //     .iter()
    //     .filter(|p| {
    //         if p.is_none() {
    //             return false;
    //         }

    //         let p = p.unwrap();

    //         !p.locked
    //     })
    //     .count();

    // dbg!(&unlocked_count);

    let result = bfs(&grid, source, destination) + total_rotations;

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
        let result = part_one("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("");
        assert_eq!(result, None);
    }
}

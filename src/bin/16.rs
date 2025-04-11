use codepage_437::{CP437_CONTROL, FromCp437};
use i18n_puzzles::utils::{
    grid2d::Grid2D,
    vector2d::{DOWN, LEFT, RIGHT, UP, Vector2D},
};
use std::{env, fs};

i18n_puzzles::solution!(16);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Connection {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Orientation {
    connections: [Option<Connection>; 4],
    rotations: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pipe {
    possible_orientations: Vec<Orientation>,
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

fn print_grid(grid: &Grid2D<Option<Pipe>>) {
    for y in 0..grid.height {
        let mut line = String::new();
        for x in 0..grid.width {
            let pos = y * grid.width + x;
            if let Some(pipe) = &grid.bytes[pos as usize] {
                if pipe.possible_orientations.len() > 1 {
                    line.push('#');
                } else {
                    line.push(connections_to_char(
                        &pipe.possible_orientations.first().unwrap().connections,
                    ));
                }
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
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
                let mut orientations: Vec<Orientation> = vec![];
                let mut rotated = connections.clone();

                for i in 0..4 {
                    if orientations
                        .iter()
                        .find(|o| o.connections == rotated)
                        .is_some()
                    {
                        continue;
                    }

                    orientations.push(Orientation {
                        connections: rotated.clone(),
                        rotations: i,
                    });
                    rotated.rotate_right(1);
                }

                pipes.push(Some(Pipe {
                    possible_orientations: orientations,
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

fn lock_grid(grid: &mut Grid2D<Option<Pipe>>) -> bool {
    let mut work_done = false;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Vector2D::new(x, y);

            let pipe = grid[point].clone();
            if pipe.is_none() {
                continue;
            }

            let mut pipe = pipe.unwrap();

            if pipe.possible_orientations.len() == 1 {
                continue;
            }

            let mut new_possible_orientations = vec![];

            'outer: for orientation in &pipe.possible_orientations {
                // Check each connection
                for (i, connection) in orientation.connections.iter().enumerate() {
                    let dir = index_to_dir(i);
                    let neighbor_point = point + dir;

                    if connection.is_none() {
                        // Make sure no neighbor is pointing here
                        if !(neighbor_point.x < 0
                            || neighbor_point.x >= grid.width
                            || neighbor_point.y < 0
                            || neighbor_point.y >= grid.height)
                        {
                            let neighbor = grid[neighbor_point].clone();

                            if let Some(neighbor) = neighbor {
                                if neighbor.possible_orientations.len() == 1 {
                                    let neighbor_connections =
                                        neighbor.possible_orientations[0].connections;
                                    let opposite = match i {
                                        0 => 2,
                                        1 => 3,
                                        2 => 0,
                                        3 => 1,
                                        _ => unreachable!(),
                                    };

                                    if neighbor_connections[opposite].is_some() {
                                        continue 'outer;
                                    }
                                }
                            }
                        }

                        continue;
                    }

                    let connection = connection.unwrap();

                    // If neighbor is out of bounds, we can't match
                    if neighbor_point.x < 0
                        || neighbor_point.x >= grid.width
                        || neighbor_point.y < 0
                        || neighbor_point.y >= grid.height
                    {
                        continue 'outer;
                    }

                    let neighbor = grid[neighbor_point].clone();

                    // If there's no neighbor pipe, we can't match
                    if neighbor.is_none() {
                        continue 'outer;
                    }

                    let neighbor = neighbor.unwrap();

                    // Check if any rotation of the neighbor
                    // can be a match
                    let mut any = 0;
                    for orientation_n in neighbor.possible_orientations {
                        let opposite = match i {
                            0 => 2,
                            1 => 3,
                            2 => 0,
                            3 => 1,
                            _ => unreachable!(),
                        };

                        let neighbor_connection = orientation_n.connections[opposite];

                        if let Some(neighbor_connection) = neighbor_connection {
                            if connection == neighbor_connection {
                                any += 1;
                            }
                        }
                    }

                    if any == 0 {
                        continue 'outer;
                    }
                }

                new_possible_orientations.push(orientation.clone());
            }

            if pipe.possible_orientations.len() != new_possible_orientations.len() {
                work_done = true;
            }

            pipe.possible_orientations = new_possible_orientations;

            grid[point] = Some(pipe);
        }
    }

    work_done
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
    let example = false;

    let cwd = env::current_dir().unwrap();
    let filepath = if !example {
        cwd.join("data")
            .join("inputs")
            .join(format!("{DAY}_real.txt"))
    } else {
        cwd.join("data").join("examples").join(format!("{DAY}.txt"))
    };
    let bytes = fs::read(&filepath).expect("Failed to read file");
    let mut string = String::from_cp437(bytes, &CP437_CONTROL);

    if !example {
        string = remove_frame(string);
    }

    let mut grid = parse_string(&string);

    // Lock source and destination
    let source_pipe = grid.bytes[0].as_mut().unwrap();
    source_pipe.possible_orientations = vec![source_pipe.possible_orientations[0]];
    let destination_pipe = grid.bytes[(grid.width * grid.height - 1) as usize]
        .as_mut()
        .unwrap();
    destination_pipe.possible_orientations = vec![destination_pipe.possible_orientations[0]];

    while lock_grid(&mut grid) {}

    print_grid(&grid);

    let result: u32 = grid
        .bytes
        .iter()
        .flatten()
        .map(|p| {
            let o = p.possible_orientations[0];

            o.rotations
        })
        .sum();

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

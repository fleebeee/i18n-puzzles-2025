use std::collections::VecDeque;

use hashbrown::HashSet;
use i18n_puzzles::utils::vector2d::{DOWN, LEFT, RIGHT, UP, Vector2D};
use itertools::Itertools;

i18n_puzzles::solution!(17);

const BIG_ENOUGH: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Neighbor {
    Unknown,
    Found(usize),
    Never,
}

#[derive(Debug, Clone)]
struct Piece {
    id: usize,
    bytes: Vec<u8>,
    width: usize,
    height: usize,
    // How many bytes start with 10 for each row
    left: Vec<usize>,
    // How many are expected from the right neighbor
    right: Vec<usize>,
    neighbors: Vec<Neighbor>,
}

// impl Piece {
//     fn print(&self) {
//         for i in 0..self.height {
//             let start = i * self.width;
//             let end = start + self.width;
//             let row = &self.bytes[start..end];
//             // println!("{:08b}", row.last().unwrap());
//             println!("{}", String::from_utf8_lossy(row));
//         }
//     }
// }

fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
        .collect()
}

fn parse_piece(input: &[&str], id: &mut usize) -> Piece {
    let mut bytes = vec![];
    let mut left = vec![];
    let mut right = vec![];

    for line in input {
        let line_bytes = hex_to_bytes(line);

        // Count how many bytes start with 10
        let l = line_bytes
            .iter()
            .position(|b| match b {
                0b10_000000..=0b10_111111 => false,
                _ => true,
            })
            .unwrap();

        let mut reverse_bytes = line_bytes.iter().rev();
        let last_byte = reverse_bytes.next().unwrap();

        // Count how many bytes more are needed to fill the last UTF-8 character
        let r = match last_byte {
            0b110_00000..=0b110_11111 => 1,
            0b1110_0000..=0b1110_1111 => 2,
            0b11110_000..=0b11110_111 => 3,
            // If the UTF-8 sequence starts before the last byte
            0b10_000000..=0b10_111111 => {
                let mut d = 1;
                let mut r = 0;
                while let Some(current) = reverse_bytes.next() {
                    if *current >= 0b10_000000 && *current <= 0b10_111111 {
                        d += 1;
                        continue;
                    }

                    r = match *current {
                        0b110_00000..=0b110_11111 => 1 - d,
                        0b1110_0000..=0b1110_1111 => 2 - d,
                        0b11110_000..=0b11110_111 => 3 - d,
                        _ => unreachable!(),
                    };

                    break;
                }

                r
            }
            _ => 0,
        };

        left.push(l);
        right.push(r);

        bytes.extend(line_bytes);
    }

    let piece = Piece {
        id: *id,
        width: bytes.len() / left.len(),
        height: left.len(),
        bytes,
        left,
        right,
        neighbors: vec![
            Neighbor::Unknown,
            Neighbor::Unknown,
            Neighbor::Unknown,
            Neighbor::Unknown,
        ],
    };

    *id += 1;

    piece
}

fn parse_input(input: &str) -> Vec<Piece> {
    let mut pieces: Vec<Piece> = vec![];
    let mut id = 0;

    for piece_str in input.split("\n\n") {
        // Break bigger pieces into chunks of 8 lines (4 if solving example)
        for (i, shard) in piece_str.lines().chunks(4).into_iter().enumerate() {
            let lines: Vec<&str> = shard.collect();
            let mut piece = parse_piece(&lines, &mut id);

            if i > 0 {
                let l = pieces.len();
                pieces[l - 1].neighbors[1] = Neighbor::Found(id - 1);
                piece.neighbors[3] = Neighbor::Found(id - 2);
            }

            // Check for each edge of the piece
            // if border characters are present and
            // restrict neighbors

            // Top: -═
            // Side: |║

            // Just checking the doubles suffices

            // ═
            for i in 0..piece.width - 2 {
                if piece.bytes[i..i + 3] == [0xe2, 0x95, 0x90] {
                    piece.neighbors[3] = Neighbor::Never;
                    break;
                }
            }
            for i in (piece.height - 1) * piece.width..piece.height * piece.width - 2 {
                if piece.bytes[i..i + 3] == [0xe2, 0x95, 0x90] {
                    piece.neighbors[1] = Neighbor::Never;
                    break;
                }
            }

            // ║
            for i in 0..piece.height {
                if piece.bytes[i * piece.width..i * piece.width + 3] == [0xe2, 0x95, 0x91] {
                    piece.neighbors[2] = Neighbor::Never;
                    break;
                }
            }
            for i in 0..piece.height {
                if piece.bytes[(i + 1) * piece.width - 3..(i + 1) * piece.width]
                    == [0xe2, 0x95, 0x91]
                {
                    piece.neighbors[0] = Neighbor::Never;
                    break;
                }
            }

            pieces.push(piece);
        }
    }

    pieces
}

fn find_neighbors(pieces: &mut Vec<Piece>) {
    for i in 0..pieces.len() {
        for j in 0..pieces.len() {
            // "UTF-8 boundaries" must match
            if i == j || pieces[i].right != pieces[j].left {
                continue;
            }

            // Only match these pieces if they're not matched already
            match pieces[i].neighbors[0] {
                Neighbor::Unknown => (),
                _ => continue,
            }
            match pieces[j].neighbors[2] {
                Neighbor::Unknown => (),
                _ => continue,
            }

            // Borders must match
            if pieces[i].neighbors[1] == Neighbor::Never
                && pieces[j].neighbors[1] != Neighbor::Never
                || pieces[i].neighbors[1] != Neighbor::Never
                    && pieces[j].neighbors[1] == Neighbor::Never
                || pieces[i].neighbors[3] == Neighbor::Never
                    && pieces[j].neighbors[3] != Neighbor::Never
                || pieces[i].neighbors[3] != Neighbor::Never
                    && pieces[j].neighbors[3] == Neighbor::Never
            {
                continue;
            }

            pieces[i].neighbors[0] = Neighbor::Found(j);
            pieces[j].neighbors[2] = Neighbor::Found(i);
            break;
        }
    }
}

fn fill_map<'a>(pieces: &'a Vec<Piece>, left_top: &'a Piece) -> Vec<Vec<Option<&'a Piece>>> {
    let mut map = vec![vec![None; BIG_ENOUGH]; BIG_ENOUGH];
    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back((left_top, Vector2D::new(0, 0)));

    while let Some((current, position)) = frontier.pop_front() {
        if visited.contains(&current.id) {
            continue;
        }

        visited.insert(current.id);

        map[position.y as usize][position.x as usize] = Some(current);

        for (i, n) in current.neighbors.iter().enumerate() {
            match n {
                Neighbor::Found(n) => {
                    let dir = match i {
                        0 => RIGHT,
                        1 => DOWN,
                        2 => LEFT,
                        3 => UP,
                        _ => unreachable!(),
                    };

                    let next = position + dir;
                    let piece_n = &pieces[*n];

                    frontier.push_back((piece_n, next));
                }
                _ => (),
            }
        }
    }

    map
}

fn stitch_together(map: &[Vec<Option<&Piece>>]) -> Vec<String> {
    let mut stitched: Vec<String> = vec![];
    let w = map[0].iter().position(|o| o.is_none()).unwrap();
    let h = map.iter().position(|o| o[0].is_none()).unwrap();
    let ph = map[0][0].unwrap().height;

    for y in 0..h {
        for y2 in 0..ph {
            let mut row = vec![];

            for x in 0..w {
                let p = map[y][x].unwrap();
                let bytes = &p.bytes;

                let start = p.width * y2;
                let end = start + p.width;
                row.extend_from_slice(&bytes[start..end]);
            }

            stitched.push(String::from_utf8_lossy(&row).to_string());
        }
    }

    stitched
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut pieces = parse_input(input);

    // Fill in neighbor data
    find_neighbors(&mut pieces);

    // ╔ marks the left top piece
    let left_top = pieces
        .iter()
        .find(|p| p.bytes[..3] == [0xe2, 0x95, 0x94])
        .unwrap();

    // BFS to map pieces to coordinates
    let map = fill_map(&pieces, left_top);

    // Visually check which jigsaw pieces we managed to put together
    // for y in 0..BIG_ENOUGH {
    //     for x in 0..BIG_ENOUGH {
    //         if map[y][x].is_some() {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // If matched each piece, stitch them together
    let stitched = stitch_together(&map);

    for y in 0..stitched.len() {
        if let Some(x) = stitched[y].chars().position(|c| c == '╳') {
            return Some((x * y) as u64);
        }
    }

    None
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
        assert_eq!(result, Some(132));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

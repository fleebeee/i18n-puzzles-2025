use crate::utils::vector2d::Vector2D;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid2D<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid2D<char> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().flatten().for_each(|ch| bytes.push(*ch));
        Grid2D {
            width,
            height,
            bytes,
        }
    }

    pub fn print(&self, focus: Option<Vector2D>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Vector2D::new(x, y);

                if let Some(focus) = focus {
                    if focus == point {
                        print!("@");
                        continue;
                    }
                }
                print!("{}", self[point] as char);
            }
            println!();
        }
        println!();
    }

    #[inline]
    pub fn orthogonal_neighbors(&self, point: &Vector2D) -> Vec<Vector2D> {
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|p| p.x >= 0 && p.y >= 0 && p.x < self.width && p.y < self.height)
            .collect()
    }
}

impl<T> Index<Vector2D> for Grid2D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Vector2D) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Vector2D> for Grid2D<T> {
    #[inline]
    fn index_mut(&mut self, index: Vector2D) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}

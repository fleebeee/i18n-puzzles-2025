use crate::utils::vector3d::Vector3D;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid3D<T> {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub bytes: Vec<T>,
}

impl Grid3D<bool> {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        let bytes = vec![false; (width * height * depth) as usize];

        Grid3D {
            width,
            height,
            depth,
            bytes,
        }
    }

    pub fn print_grid3d(&self) {
        let w = self.width as usize;
        let h = self.height as usize;
        let d = self.depth as usize;

        // XY plane
        println!("XY plane:");
        for z in 0..d {
            println!("z = {}", z);
            for y in 0..h {
                for x in 0..w {
                    let c = if self[Vector3D::new(x as i32, y as i32, z as i32)] {
                        '#'
                    } else {
                        '.'
                    };
                    print!("{c}");
                }
                println!();
            }
            println!();
        }

        // XZ plane
        println!("XZ plane:");
        for y in 0..h {
            println!("y = {}", y);
            for z in 0..d {
                for x in 0..w {
                    let c = if self[Vector3D::new(x as i32, y as i32, z as i32)] {
                        '#'
                    } else {
                        '.'
                    };
                    print!("{c}");
                }
                println!();
            }
            println!();
        }

        // YZ plane
        println!("YZ plane:");
        for x in 0..w {
            println!("x = {}", x);
            for z in 0..d {
                for y in 0..h {
                    let c = if self[Vector3D::new(x as i32, y as i32, z as i32)] {
                        '#'
                    } else {
                        '.'
                    };
                    print!("{c}");
                }
                println!();
            }
            println!();
        }
    }
}

impl<T> Grid3D<T> {
    #[inline]
    pub fn orthogonal_neighbors(&self, point: &Vector3D) -> Vec<Vector3D> {
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|p| {
                p.x >= 0
                    && p.y >= 0
                    && p.z >= 0
                    && p.x < self.width
                    && p.y < self.height
                    && p.z < self.depth
            })
            .collect()
    }
}

impl<T> Index<Vector3D> for Grid3D<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Vector3D) -> &Self::Output {
        let idx = (index.x + self.width * index.y + self.width * self.height * index.z) as usize;
        &self.bytes[idx]
    }
}

impl<T> IndexMut<Vector3D> for Grid3D<T> {
    #[inline]
    fn index_mut(&mut self, index: Vector3D) -> &mut Self::Output {
        let idx = (index.x + self.width * index.y + self.width * self.height * index.z) as usize;
        &mut self.bytes[idx]
    }
}

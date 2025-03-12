use hashbrown::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Vector2D::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Vector2D::new(self.y, -self.x)
    }

    #[inline]
    pub fn orthogonal_neighbors(self) -> Vec<Self> {
        vec![
            Vector2D::new(self.x + 1, self.y),
            Vector2D::new(self.x, self.y + 1),
            Vector2D::new(self.x - 1, self.y),
            Vector2D::new(self.x, self.y - 1),
        ]
    }

    #[inline]
    pub fn manhattan(self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    pub fn signum(self, other: Self) -> Self {
        Vector2D::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }

    #[inline]
    pub fn wrap(self, size: &Self) -> Self {
        Vector2D::new((self.x + size.x) % size.x, (self.y + size.y) % size.y)
    }
}

impl Hash for Vector2D {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Vector2D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Vector2D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Vector2D::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Vector2D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub const ORIGIN: Vector2D = Vector2D::new(0, 0);
pub const UP: Vector2D = Vector2D::new(0, -1);
pub const DOWN: Vector2D = Vector2D::new(0, 1);
pub const LEFT: Vector2D = Vector2D::new(-1, 0);
pub const RIGHT: Vector2D = Vector2D::new(1, 0);
pub const ORTHOGONAL: [Vector2D; 4] = [UP, DOWN, LEFT, RIGHT];
pub const NE: Vector2D = Vector2D::new(1, -1);
pub const SE: Vector2D = Vector2D::new(1, 1);
pub const SW: Vector2D = Vector2D::new(-1, 1);
pub const NW: Vector2D = Vector2D::new(-1, -1);
// Reading order
pub const DIAGONAL: [Vector2D; 8] = [NW, UP, NE, LEFT, RIGHT, SW, DOWN, SE];

pub fn get_vector2d_vec_bounds(vecs: &[Vector2D]) -> (Vector2D, Vector2D) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;

    for vec in vecs.iter() {
        x_min = x_min.min(vec.x);
        x_max = x_max.max(vec.x);
        y_min = y_min.min(vec.y);
        y_max = y_max.max(vec.y);
    }

    (Vector2D::new(x_min, y_min), Vector2D::new(x_max, y_max))
}

pub fn print_vector2d_set(s: &HashSet<Vector2D>) {
    let vecs: Vec<Vector2D> = s.iter().cloned().collect();
    let (min, max) = get_vector2d_vec_bounds(&vecs);

    let w = (max.x - min.x) as usize + 1;
    let h = (max.y - min.y) as usize + 1;
    let mut map = vec![vec![false; w]; h];

    for vec in vecs {
        let x = (vec.x - min.x) as usize;
        let y = (vec.y - min.y) as usize;
        map[y][x] = true;
    }

    for y in 0..h {
        for x in 0..w {
            let c = match map[y][x] {
                true => '#',
                false => '.',
            };

            print!("{c}");
        }
        println!();
    }
    println!();
}

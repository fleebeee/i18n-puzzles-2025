use hashbrown::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn orthogonal_neighbors(self) -> Vec<Self> {
        vec![
            Vector3D::new(self.x + 1, self.y, self.z),
            Vector3D::new(self.x, self.y + 1, self.z),
            Vector3D::new(self.x - 1, self.y, self.z),
            Vector3D::new(self.x, self.y - 1, self.z),
            Vector3D::new(self.x, self.y, self.z + 1),
            Vector3D::new(self.x, self.y, self.z - 1),
        ]
    }

    #[inline]
    pub fn manhattan(self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    #[inline]
    pub fn signum(self, other: Self) -> Self {
        Vector3D::new(
            (self.x - other.x).signum(),
            (self.y - other.y).signum(),
            (self.z - other.z).signum(),
        )
    }
}

impl Hash for Vector3D {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.write_i32(self.z);
    }
}

impl Add for Vector3D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<i32> for Vector3D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sub for Vector3D {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

pub fn print_vector3d_set(s: &HashSet<Vector3D>) {
    let s: Vec<_> = s.iter().cloned().collect();

    print_vector3d_vec(&s);
}

pub fn get_vector3d_vec_bounds(vec: &[Vector3D]) -> (Vector3D, Vector3D) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    let mut z_min = i32::MAX;
    let mut z_max = i32::MIN;

    for v in vec {
        x_min = x_min.min(v.x);
        x_max = x_max.max(v.x);
        y_min = y_min.min(v.y);
        y_max = y_max.max(v.y);
        z_min = z_min.min(v.z);
        z_max = z_max.max(v.z);
    }

    (
        Vector3D::new(x_min, y_min, z_min),
        Vector3D::new(x_max, y_max, z_max),
    )
}

pub fn print_vector3d_vec(vecs: &[Vector3D]) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    let mut z_min = i32::MAX;
    let mut z_max = i32::MIN;

    for vec in vecs.iter() {
        x_min = x_min.min(vec.x);
        x_max = x_max.max(vec.x);
        y_min = y_min.min(vec.y);
        y_max = y_max.max(vec.y);
        z_min = z_min.min(vec.z);
        z_max = z_max.max(vec.z);
    }

    let w = (x_max - x_min) as usize + 1;
    let h = (y_max - y_min) as usize + 1;
    let d = (z_max - z_min) as usize + 1;

    // XY plane
    let mut map_xy = vec![vec![false; w]; h];
    for vec in vecs.iter() {
        let x = (vec.x - x_min) as usize;
        let y = (vec.y - y_min) as usize;
        map_xy[y][x] = true;
    }

    println!("XY plane:");
    for y in 0..h {
        for x in 0..w {
            let c = match map_xy[y][x] {
                true => '#',
                false => '.',
            };
            print!("{c}");
        }
        println!();
    }
    println!();

    // XZ plane
    let mut map_xz = vec![vec![false; w]; d];
    for vec in vecs.iter() {
        let x = (vec.x - x_min) as usize;
        let z = (vec.z - z_min) as usize;
        map_xz[z][x] = true;
    }

    println!("XZ plane:");
    for z in 0..d {
        for x in 0..w {
            let c = match map_xz[z][x] {
                true => '#',
                false => '.',
            };
            print!("{c}");
        }
        println!();
    }
    println!();

    // YZ plane
    let mut map_yz = vec![vec![false; h]; d];
    for vec in vecs.iter() {
        let y = (vec.y - y_min) as usize;
        let z = (vec.z - z_min) as usize;
        map_yz[z][y] = true;
    }

    println!("YZ plane:");
    for z in 0..d {
        for y in 0..h {
            let c = match map_yz[z][y] {
                true => '#',
                false => '.',
            };
            print!("{c}");
        }
        println!();
    }
    println!();
}

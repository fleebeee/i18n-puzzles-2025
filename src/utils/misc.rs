use std::ops::{Div, Mul, Rem};

pub fn lcm<T>(nums: &[T]) -> T
where
    T: Copy
        + From<u8>
        + PartialOrd
        + PartialEq
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>,
{
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + From<u8> + PartialOrd + Rem<Output = T>,
{
    if b == T::from(0u8) {
        return a;
    }
    gcd(b, a % b)
}

pub fn find_repeating_sequence<T: PartialEq>(data: &[T]) -> Option<(usize, usize)> {
    let n = data.len();

    // Try different sequence lengths
    for len in 1..=n / 2 {
        let mut is_repeating = true;

        // Check if the sequence repeats
        for i in 0..n - len {
            if data[i] != data[i + len] {
                is_repeating = false;
                break;
            }
        }

        if is_repeating {
            return Some((0, len));
        }
    }

    None
}

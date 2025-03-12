use std::str::FromStr;

#[inline]
fn is_number(b: u8) -> bool {
    b >= b'0' && b <= b'9'
}

pub fn parse_signed<T: FromStr>(input: &str) -> Vec<T> {
    let mut numbers = vec![];

    let len = input.len();
    let mut i = 0;

    let input = input.as_bytes();

    while i < len {
        while i < len && !is_number(input[i]) && input[i] != b'-' {
            i += 1;
        }

        if i == len {
            return numbers;
        }

        let start = i;
        i += 1;

        while i < len && is_number(input[i]) {
            i += 1;
        }

        let number = std::str::from_utf8(&input[start..i]).unwrap();
        let number = number.parse::<T>();

        match number {
            Ok(n) => numbers.push(n),
            Err(_) => println!("Parsing failed"),
        }
    }

    numbers
}

pub fn parse_unsigned<T: FromStr>(input: &str) -> Vec<T> {
    let mut numbers = vec![];

    let len = input.len();
    let mut i = 0;

    let input = input.as_bytes();

    while i < len {
        while i < len && !is_number(input[i]) {
            i += 1;
        }

        if i == len {
            return numbers;
        }

        let start = i;
        i += 1;

        while i < len && is_number(input[i]) {
            i += 1;
        }

        let number = std::str::from_utf8(&input[start..i]).unwrap();
        let number = number.parse::<T>();

        match number {
            Ok(n) => numbers.push(n),
            Err(_) => println!("Parsing failed"),
        }
    }

    numbers
}

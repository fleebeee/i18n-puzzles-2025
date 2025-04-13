i18n_puzzles::solution!(18);
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BIDI_RE: Regex = Regex::new(r"[\u2066\u2067\u2069]").unwrap();
}

#[derive(Debug)]
enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Number(i64),
}

fn strip_bidi(line: &str) -> String {
    BIDI_RE.replace_all(line, "").to_string()
}

fn parse_side(bytes: &[u8]) -> (Expression, usize) {
    // Either match a number or a nested expression
    match bytes[0] {
        b'0'..=b'9' => {
            let mut n = 0i64;
            let mut i = 0;

            while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
                n *= 10;
                n += (bytes[i] - b'0') as i64;
                i += 1;
            }

            (Expression::Number(n), i)
        }
        b'(' => {
            let mut i = 0;
            let mut depth = 1;

            while depth > 0 {
                i += 1;
                match bytes[i] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => (),
                }
            }

            (parse_expression(&bytes[1..i]), i + 1)
        }
        _ => unreachable!(),
    }
}

fn parse_expression(bytes: &[u8]) -> Expression {
    let (left, i) = parse_side(bytes);
    let operator = bytes[i + 1];
    let (right, _) = parse_side(&bytes[i + 3..]);

    match operator {
        b'+' => Expression::Add(Box::new(left), Box::new(right)),
        b'-' => Expression::Sub(Box::new(left), Box::new(right)),
        b'*' => Expression::Mul(Box::new(left), Box::new(right)),
        b'/' => Expression::Div(Box::new(left), Box::new(right)),
        _ => unreachable!(),
    }
}

// Ideally we'd store the numbers as fractions, not floats
fn compute(expression: &Expression) -> f64 {
    match expression {
        Expression::Number(n) => *n as f64,
        Expression::Add(left, right) => compute(left) + compute(right),
        Expression::Sub(left, right) => compute(left) - compute(right),
        Expression::Mul(left, right) => compute(left) * compute(right),
        Expression::Div(left, right) => compute(left) / compute(right),
    }
}

fn flip_highest_stretch(chars: &mut Vec<char>, levels: &mut Vec<u8>) {
    let max = *levels.iter().max().unwrap();

    let start = levels.iter().position(|l| *l == max).unwrap();
    let end = start
        + levels[start..]
            .iter()
            .position(|l| *l != max)
            .unwrap_or(chars.len() - start);

    // Ignore stretches of length 1
    if end - start == 1 {
        levels[start] -= 1;
        return;
    }

    let mut flipped = vec![];

    for i in (start..end).rev() {
        let c = match chars[i] {
            '(' => ')',
            ')' => '(',
            other => other,
        };

        flipped.push(c);
    }

    for i in start..end {
        levels[i] -= 1;
        chars[i] = flipped[i - start];
    }
}

fn flip(line: &str) -> String {
    // Determine embedding level per character
    let mut levels = vec![];
    let mut level = 0;

    let mut chars: Vec<char> = line.chars().collect();

    for c in &chars {
        if c.is_numeric() && level % 2 != 0 {
            levels.push(level + 1);
        } else {
            levels.push(level);
        }

        match c {
            '\u{2066}' => level += 1,
            '\u{2067}' => level += 1,
            '\u{2069}' => level -= 1,
            _ => (),
        }
    }

    // Identify highest stretches and flip them
    while levels.iter().any(|l| *l > 0) {
        flip_highest_stretch(&mut chars, &mut levels);
    }

    chars.iter().collect::<String>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rex: Vec<String> = input.lines().map(|l| strip_bidi(l)).collect();
    let lynx: Vec<String> = input.lines().map(|l| strip_bidi(&flip(l))).collect();
    let mut total = 0;

    for i in 0..rex.len() {
        let rex_value = compute(&parse_expression(&rex[i].as_bytes()));
        let lynx_value = compute(&parse_expression(&lynx[i].as_bytes()));
        total += (rex_value.round() as u64).abs_diff(lynx_value.round() as u64);
    }

    Some(total)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

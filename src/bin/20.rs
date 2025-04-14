i18n_puzzles::solution!(20);

fn decode_base64(input: &str) -> Vec<u8> {
    input
        .trim()
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => None,
            b'\n' => None,
            _ => unreachable!(),
        })
        .collect()
}

fn unpack_base64(base64: &[u8]) -> Vec<u8> {
    let mut bytes = vec![];

    // Every 4 base64 characters map to 3 bytes
    for chunk in base64.chunks_exact(4) {
        let first = (chunk[0] << 2) | (chunk[1] >> 4);
        let second = (chunk[1] << 4) | (chunk[2] >> 2);
        let third = (chunk[2] << 6) | chunk[3];

        bytes.push(first);
        bytes.push(second);
        bytes.push(third);
    }

    bytes
}

fn to_utf16_le(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|c| ((c[1] as u16) << 8) + c[0] as u16)
        .collect()
}

fn unpack_utf16_le(bytes: &[u16]) -> Vec<u8> {
    let mask = 0b1111111111;
    // Extract 20 bit sequences from UTF-16 surrogate pairs
    let mut big_bytes: Vec<u32> = vec![];

    let mut i = 0;
    while i < bytes.len() {
        let big_byte = match bytes[i] {
            // Extract code point from surrogate pair
            // like so:
            // 110110_aaaaaaaaaa
            // 110111_bbbbbbbbbb
            0b110110_0000000000..=0b110110_1111111111 => {
                // Real input seems to have a dangling surrogate pair?
                if i + 1 == bytes.len() {
                    break;
                }

                let a = (bytes[i] as u32 & mask) << 10;
                let b = bytes[i + 1] as u32 & mask;
                i += 2;

                (a | b) + 0x10000
            }
            // Anything else gets treated as a 20 bit sequence
            _ => {
                let a = bytes[i] as u32;
                i += 1;

                a
            }
        };

        big_bytes.push(big_byte);
    }

    // Split two u32 into five u8 (each u32 holds 20 bits)
    big_bytes
        .chunks_exact(2)
        .map(|c| {
            let mut bytes = vec![];
            let both = ((c[0] as u64) << 20) | (c[1] as u64);

            for i in 0..5 {
                bytes.push(((both >> (i * 8)) & 0xFF) as u8);
            }

            bytes.reverse();
            bytes
        })
        .flatten()
        .collect()
}

fn handle_utf8_bytes(bytes: &[u8], length: usize) -> u32 {
    let mut big_byte: u32 = 0;

    for i in (1..length).rev() {
        big_byte |= (bytes[i] as u32 & 0b111111) << ((length - 1 - i) * 6);
    }

    big_byte
}

fn extract_code_points_from_invalid_utf8(bytes: &[u8]) -> Vec<u32> {
    // This spec seems to allow up to 6 byte long sequences
    // e.g.
    // 1111110_0
    // 10_001110
    // 10_101010
    // 10_101010
    // 10_101010
    // 10_101110

    // Extract code points
    let mut code_points = vec![];

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            // Length 6
            0b111111_00..=0b111111_11 => {
                let l = 6;
                // There's one truncated entry near the end?
                if i + l >= bytes.len() {
                    return code_points;
                }

                let mut big_byte = handle_utf8_bytes(&bytes[i..i + l], l);
                big_byte |= ((bytes[i] & 0b11) as u32) << ((l - 1) * 6);
                code_points.push(big_byte);
                i += l;
            }
            // Length 5
            0b11111_000..=0b11111_111 => {
                let l = 5;

                let mut big_byte = handle_utf8_bytes(&bytes[i..i + l], l);
                big_byte |= ((bytes[i] & 0b111) as u32) << ((l - 1) * 6);
                code_points.push(big_byte);
                i += l;
            }
            // The data only has the above cases
            _ => i += 1,
        }
    }

    code_points
}

fn extract_utf8_bytes_from_code_points(code_points: &[u32]) -> Vec<u8> {
    let mut bytes = vec![];

    // Every 2 code points corresponds to 7 bytes
    for i in (0..code_points.len()).step_by(2) {
        if i + 1 == code_points.len() {
            return bytes;
        }

        let first = code_points[i];
        let second = code_points[i + 1];
        // 0000aaaaaaaabbbbbbbbccccccccdddd
        // 0000ddddeeeeeeeeffffffffgggggggg
        let a = ((first >> 20) & 0xFF) as u8;
        let b = ((first >> 12) & 0xFF) as u8;
        let c = ((first >> 4) & 0xFF) as u8;
        let d = (((first << 4) | ((second >> 24) & 0xFF)) & 0xFF) as u8;
        let e = ((second >> 16) & 0xFF) as u8;
        let f = ((second >> 8) & 0xFF) as u8;
        let g = (second & 0xFF) as u8;

        bytes.extend([a, b, c, d, e, f, g]);
    }

    bytes
}

pub fn part_one(input: &str) -> Option<String> {
    let base64 = decode_base64(input);
    let unpacked = unpack_base64(&base64);

    // Skip BOM
    let utf16_le_bytes = to_utf16_le(&unpacked[2..]);

    let invalid_utf8 = unpack_utf16_le(&utf16_le_bytes);
    let code_points = extract_code_points_from_invalid_utf8(&invalid_utf8);
    let utf8_bytes = extract_utf8_bytes_from_code_points(&code_points);

    let answer = String::from_utf8_lossy(&utf8_bytes).to_string();

    Some(answer)
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

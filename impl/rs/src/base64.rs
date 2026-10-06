const BASE64_CHARS: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Base64 encode a slice of bytes
pub fn base64_encode(input: String) -> String {
    let mut result = String::new();
    let mut buffer: u32 = 0;
    let mut bits_left = 0;

    for byte in input.into_bytes() {
        buffer = (buffer << 8) | u32::from(byte);
        bits_left += 8;

        while bits_left >= 6 {
            bits_left -= 6;
            let index = ((buffer >> bits_left) & 0b11_1111) as usize;
            result.push(BASE64_CHARS[index] as char);
        }
    }

    if bits_left > 0 {
        buffer <<= 6 - bits_left;
        let index = (buffer & 0b11_1111) as usize;
        result.push(BASE64_CHARS[index] as char);
    }

    while !result.len().is_multiple_of(4) {
        result.push('=');
    }

    result
}

// Base64 decode a string into a vector of bytes
#[allow(clippy::cast_possible_truncation)]
pub fn base64_decode(encoded: &str) -> Option<Vec<u8>> {
    let mut decoded = Vec::new();
    let mut padding = 0;
    let mut buffer = 0;
    let mut bits = 0;

    for c in encoded.chars() {
        let value = match BASE64_CHARS.iter().position(|&x| x == c as u8) {
            Some(v) => v as u32,
            None if c == '=' => {
                padding += 1;
                continue;
            }
            None => return None,
        };

        // data after padding is malformed
        if padding > 0 {
            return None;
        }

        buffer = (buffer << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            decoded.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }

    // padding is optional, but when present it must match the leftover bits
    let valid_padding = match padding {
        0 => true,
        1 => bits == 2,
        2 => bits == 4,
        _ => false,
    };

    if bits >= 6 || !valid_padding {
        return None;
    }

    Some(decoded)
}

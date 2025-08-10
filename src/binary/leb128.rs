#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leb128Err {
    Unterminated,
    Overflow,
}

#[inline]
pub fn decode_uleb128_u64(
    input: &[u8],
    max_bytes: usize,
    max_bits: u32,
) -> Result<(u64, usize), Leb128Err> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    let mut used = 0usize;

    for &b in input.iter().take(max_bytes) {
        let low = (b & 0x7f) as u64;
        result = result
            .checked_add(low.checked_shl(shift).ok_or(Leb128Err::Overflow)?)
            .ok_or(Leb128Err::Overflow)?;
        used += 1;

        if (b & 0x80) == 0 {
            if max_bits < 64 && (result >> max_bits) != 0 {
                return Err(Leb128Err::Overflow);
            }
            return Ok((result, used));
        }

        shift += 7;
        if shift >= max_bits {
            return Err(Leb128Err::Overflow);
        }
    }
    Err(Leb128Err::Unterminated)
}

#[inline]
pub fn decode_sleb128_i64(
    input: &[u8],
    max_bytes: usize,
    max_bits: u32,
) -> Result<(i64, usize), Leb128Err> {
    let mut result: i64 = 0;
    let mut shift: u32 = 0;
    let mut used = 0usize;

    for &b in input.iter().take(max_bytes) {
        let low = (b & 0x7f) as i64;
        result = result
            .checked_add(low.checked_shl(shift).ok_or(Leb128Err::Overflow)?)
            .ok_or(Leb128Err::Overflow)?;
        used += 1;
        shift += 7;
        if (b & 0x80) == 0 {
            if max_bits < 64 && (result >> max_bits) != 0 {
                return Err(Leb128Err::Overflow);
            }
            if (b & 0x40) != 0 {
                // Sign extend if the sign bit is set
                result |= -1i64 << shift;
            }
            return Ok((result, used));
        }
        if shift >= max_bits {
            return Err(Leb128Err::Overflow);
        }
    }
    Err(Leb128Err::Unterminated)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_uleb128_u64() {
        let input = [0xE5, 0x8E, 0x26];
        let result = decode_uleb128_u64(&input, 3, 64);
        assert_eq!(result, Ok((624485, 3)));
    }

    #[test]
    fn test_decode_uleb128_u64_overflow() {
        let input = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let result = decode_uleb128_u64(&input, 8, 64);
        assert!(result.is_err());
        matches!(result.err(), Some(Leb128Err::Overflow));
    }

    #[test]
    fn test_decode_uleb128_u64_unterminated() {
        let input = [0xE5, 0x8E];
        let result = decode_uleb128_u64(&input, 2, 64);
        assert!(result.is_err());
        matches!(result.err(), Some(Leb128Err::Unterminated));
    }

    #[test]
    fn test_decode_sleb128_i64() {
        let input = [0xC0, 0xBB, 0x78];
        let result = decode_sleb128_i64(&input, 4, 64);
        assert_eq!(result, Ok((-123456, 3)));
    }

    #[test]
    fn test_decode_sleb128_i32() {
        let input = [0xff, 0x7f];
        let result = decode_sleb128_i64(&input, 2, 16);
        assert_eq!(result, Ok((-1, 2)));
    }
}

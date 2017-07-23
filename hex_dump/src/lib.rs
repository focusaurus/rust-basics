#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn test_format_offset() {
        assert_eq!(format_offset(0), "00000000");
        assert_eq!(format_offset(1), "00000001");
        assert_eq!(format_offset(10), "0000000a");
        assert_eq!(format_offset(17), "00000011");
        assert_eq!(format_offset(65536), "00010000");
    }

    #[test]
    fn test_format_byte() {
        assert_eq!(format_byte(0), "00");
        assert_eq!(format_byte(1), "01");
        assert_eq!(format_byte(125), "7d");
        assert_eq!(format_byte(255), "ff");
    }

    #[test]
    fn test_format_in_place() {
        assert_eq!(format_in_place(0, 7), "00000000 07");
        assert_eq!(format_in_place(1, 10), " 0a");
        assert_eq!(format_in_place(15, 255), " ff");
        assert_eq!(format_in_place(16, 2), "\n00000010 02");
        assert_eq!(format_in_place(32, 11), "\n00000020 0b");
    }
}

/// Formats a numeric offset with 8 leading zeros
/// # Examples
///
/// ```
/// assert_eq!(hex_dump::format_offset(1), "00000001");
/// ```
pub fn format_offset(offset: usize) -> String {
    format!("{:08x}", offset)
}

/// Formats a single byte in 2 hex digits, lowercase.
/// # Examples
///
/// ```
/// assert_eq!(hex_dump::format_byte(125), "7d");
/// ```
pub fn format_byte(byte: u8) -> String {
    format!("{:02x}", byte)
}

/// Formats a single byte in 2 hex digits, in 16-byte lines.
/// This handles the offset prefix and trailing newline.
/// # Examples
///
/// ```
/// assert_eq!(hex_dump::format_in_place(32, 11), "\n00000020 0b");
/// ```
pub fn format_in_place(index: usize, byte: u8) -> String {
    // interior bytes just need space
    let mut prefix = String::from(" ");
    let offset = format_offset(index);
    if index % 16 == 0 {
        if index == 0 {
            // zero we need just offset space
            prefix = format!("{} ", offset);
        } else {
            // other rows we need newline offset space
            prefix = format!("\n{} ", offset);
        }
    }
    format!("{}{}", prefix, format_byte(byte))
}

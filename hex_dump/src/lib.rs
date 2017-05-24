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
}

fn format_offset(offset: u32) -> String {
    format!("{:08x}", offset)
}

fn format_byte(byte: u8) -> String {
    format!("{:02x}", byte)
}

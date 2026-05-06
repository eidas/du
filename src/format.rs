pub fn format_size(bytes: u64, human_readable: bool) -> String {
    if !human_readable {
        // Default: 1KB blocks (ceiling division), matching traditional du behavior
        return format!("{}", bytes.div_ceil(1024));
    }

    const UNITS: &[(&str, u64)] = &[("GB", 1 << 30), ("MB", 1 << 20), ("KB", 1 << 10)];

    for (unit, factor) in UNITS {
        if bytes >= *factor {
            return format!("{:.1} {}", bytes as f64 / *factor as f64, unit);
        }
    }

    format!("{} B", bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks() {
        assert_eq!(format_size(0, false), "0");
        assert_eq!(format_size(1024, false), "1");
        assert_eq!(format_size(1025, false), "2");
    }

    #[test]
    fn test_human_readable() {
        assert_eq!(format_size(0, true), "0 B");
        assert_eq!(format_size(512, true), "512 B");
        assert_eq!(format_size(1024, true), "1.0 KB");
        assert_eq!(format_size(1536, true), "1.5 KB");
        assert_eq!(format_size(1024 * 1024, true), "1.0 MB");
        assert_eq!(format_size(1024 * 1024 * 1024, true), "1.0 GB");
    }
}

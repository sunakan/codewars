fn largest_five_digit_number(num: &str) -> u32 {
    num.chars()
        .collect::<Vec<_>>()
        .windows(5)
        .map(|t| t.iter().collect::<String>())
        .map(|s| s.parse::<_>())
        .map(|v| v.unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
    }
}

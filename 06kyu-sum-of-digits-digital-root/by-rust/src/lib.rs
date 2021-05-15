fn naruhodo(n: i64) -> i64 {
    if n < 10 {
        n
    } else {
        naruhodo((n / 10) + (n % 10))
    }
}

fn digital_root(n: i64) -> i64 {
    let sum = n.to_string().chars().map(|c| c as i64 - 48).sum();
    if sum < 10 {
        sum
    } else {
        digital_root(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }

    #[test]
    fn returns_expected2() {
        assert_eq!(naruhodo(16), 7);
        assert_eq!(naruhodo(942), 6);
        assert_eq!(naruhodo(132189), 6);
        assert_eq!(naruhodo(493193), 2);
    }
}

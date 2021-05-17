// 3 or 5で割り切れるnum未満の整数の合計値
fn solution(num: i32) -> i32 {
    (1..num).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(solution(10), 23);
        assert_eq!(solution(11), 33);
        assert_eq!(solution(6), 8);
    }
}

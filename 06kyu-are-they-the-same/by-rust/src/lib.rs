fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a = a.iter().map(|x| x * x).collect::<Vec<_>>();
    let mut b = b;
    a.sort();
    b.sort();
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_comp_1() {
        let a = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let b = vec![
            11 * 11,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        assert_eq!(comp(a, b), true);
    }

    #[test]
    fn tests_comp_2() {
        let a = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let b = vec![
            11 * 21,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        assert_eq!(comp(a, b), false);
    }
}

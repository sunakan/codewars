//
// Rust windows (イテレーションの方)
// https://qiita.com/hystcs/items/d33e77084277cdba8052
//
// Rust By Example の タプル
// https://doc.rust-jp.rs/rust-by-example-ja/primitives/tuples.html
//
fn naruhodo(chars: &[char]) -> char {
    (chars
        .windows(2)
        .map(|w| (w[0] as u8, w[1] as u8))
        .find(|&w| w.0 + 1 != w.1)
        .unwrap()
        .0
        + 1) as char
}

fn find_missing_letter(chars: &[char]) -> char {
    for i in 0..chars.len() {
        let expected = chars[i] as u8 + 1;
        let next = chars[i + 1] as u8;
        if expected != next {
            return char::from(expected);
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }

    #[test]
    fn example_tests2() {
        assert_eq!(naruhodo(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(naruhodo(&['O', 'Q', 'R', 'S']), 'P');
    }
}

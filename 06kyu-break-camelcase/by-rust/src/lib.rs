fn naruhodo(s: &str) -> String {
    s.chars()                                // camelCase
        .rev()                               // esaClemac
        .collect::<String>()                 // esaClemac
        .split_inclusive(char::is_uppercase) // iter(esaC lemac)
        .map(|x| x.chars().rev().collect())  // Case camel
        .rev()                               // iter(camel Case)
        .collect::<Vec<String>>()            // camel, Case
        .join(" ")                           // camel Case
}

fn solution(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                format!(" {}", c)
            } else {
                format!("{}", c)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(naruhodo("camelCasing"), "camel Casing");
        assert_eq!(naruhodo("camelCasingTest"), "camel Casing Test");
    }

}

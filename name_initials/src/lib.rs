pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| name.chars().filter(|c| c.is_uppercase()))
        .map(|is| is.map(|initial| initial.to_string() + "."))
        .map(|is| {
            is.reduce(|res, initial| format!("{res} {initial}"))
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        assert_eq!(initials(names), ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}

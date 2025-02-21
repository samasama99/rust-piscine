pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| name.split(" "))
        .map(|parts| {
            parts
                .map(|part| part.chars().next().unwrap())
                .map(|initial| initial.to_string() + ".")
                .collect::<Vec<String>>()
                .join(" ")
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

// Too many allocations ??
// pub fn initials(names: Vec<&str>) -> Vec<String> {
//     names
//         .into_iter()
//         .map(|name| name.chars().filter(|c| c.is_uppercase()))
//         .map(|is| is.map(|initial| format!("{initial}.")))
//         .map(|is| is.collect::<Vec<String>>().join(" "))
//         .collect()
// }

pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            name.chars().filter(|c| c.is_uppercase()).fold(
                String::with_capacity(5),
                |mut acc, c| {
                    if !acc.is_empty() {
                        acc.push(' ');
                    }
                    acc.push(c);
                    acc.push('.');
                    acc
                },
            )
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

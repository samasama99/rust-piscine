// Complete the capitalize_first function which converts the first letter of the string to uppercase.
//
// Complete the title_case function which converts the first letter of each word in a string to uppercase.
//
// Complete the change_case function which converts the uppercase letters of a string into lowercase, and the lowercase to uppercase.

pub fn capitalize_first(input: &str) -> String {
    if input.len() <= 1 {
        input.chars().collect::<String>().to_uppercase()
    } else {
        let mut iter = input.chars();
        let first = iter.next().map(|c| c.to_uppercase()).unwrap().to_string();
        first + iter.collect::<String>().as_ref()
    }
}

pub fn title_case(input: &str) -> String {
    let (_, res) = input
        .chars()
        .fold((true, String::new()), |(first, res), c| {
            (
                c.is_whitespace(),
                if first {
                    res + c.to_uppercase().to_string().as_ref()
                } else {
                    res + c.to_lowercase().to_string().as_ref()
                },
            )
        });

    res
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next()
            } else {
                c.to_lowercase().next()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        assert_eq!("Joe is missing", capitalize_first("joe is missing"));
    }
    #[test]
    fn it_works_2() {
        assert_eq!("Jill Is Leaving A", title_case("jill is leaving A"));
    }
    #[test]
    fn it_works_3() {
        assert_eq!("HEllO thERE", change_case("heLLo THere"));
    }
}

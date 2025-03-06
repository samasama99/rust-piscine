// Complete the function num_to_ordinal. It returns the ordinal number for a given cardinal number.

pub fn num_to_ordinal(x: u32) -> String {
    match x {
        10..=19 => x.to_string() + "th",
        _ => match x % 10 {
            1 => x.to_string() + "st",
            2 => x.to_string() + "nd",
            3 => x.to_string() + "rd",
            _ => x.to_string() + "th",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("1st", num_to_ordinal(1));
        assert_eq!("22nd", num_to_ordinal(22));
        assert_eq!("43rd", num_to_ordinal(43));
        assert_eq!("47th", num_to_ordinal(47));
    }
}

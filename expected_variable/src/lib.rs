// Instructions
//
// Create a function named expected_variable that receives a string to compare and an expected string.
//      It should return an Option. Every comparison should be case insensitive.
//
// If the compared string is not in camel case or snake case, expected_variable returns None.
//      You can use the case crate to figure that out. Otherwise, the compared string should be compared to the expected string using the edit_distance function that you have already created.
//
// If the result of edit_distance has more than 50% alikeness to the expected string,
//      considering the length of the expected string and the result of edit_distance, the function should return that value with a '%' symbol after the number. Otherwise expected_value should return None.

use case::CaseExt;
use std::{iter::Peekable, str::Chars, usize};

fn helper(mut source: Peekable<Chars<'_>>, mut target: Peekable<Chars<'_>>) -> usize {
    let source_first_path = source.clone();
    let source_second_path = source.clone();
    let mut target_first_path = target.clone();
    let mut target_second_path = target.clone();

    match (source.peek(), target.peek()) {
        (None, None) => 0,
        (Some(a), Some(b)) if a == b => {
            source.next();
            target.next();
            helper(source, target)
        }
        (Some(a), Some(b)) if a != b => {
            target_first_path.next();
            *target_second_path.peek_mut().unwrap() = *a;
            1 + usize::min(
                helper(source_first_path, target_first_path),
                helper(source_second_path, target_second_path),
            )
        }
        (Some(_), None) => source.count(),
        (None, Some(_)) => target.count(),
        _ => unreachable!(),
    }
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let source = source.chars().peekable();
    let target = target.chars().peekable();

    helper(source, target)
}

pub fn expected_variable(compared: &str, expected_string: &str) -> Option<String> {
    dbg!(compared);
    dbg!(expected_string);
    println!("{}", compared);
    println!("{}", expected_string);
    if compared.len() == 0 || expected_string.len() == 0 {
        return None;
    }
    let compared = compared.to_lowercase();
    let expected_string = expected_string.to_lowercase();
    if !((&compared).to_snake() == compared) && !((&compared).to_camel() == compared) {
        None
    } else {
        let percentage = ((expected_string.len() as f64
            - edit_distance(&compared, &expected_string) as f64)
            / expected_string.len() as f64
            * 100.0)
            .ceil() as usize;
        if percentage <= 50 {
            None
        } else {
            Some(format!("{}%", percentage))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("100%", expected_variable("On_Point", "on_point").unwrap());
        assert_eq!("88%", expected_variable("soClose", "so_close").unwrap());
        assert_eq!(
            None,
            expected_variable("something", "something_completely_different")
        );
        assert_eq!(
            "67%",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}

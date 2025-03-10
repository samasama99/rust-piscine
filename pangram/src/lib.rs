// Create a function named is_pangram which returns true if the given string is a pangram.
// A pangram is a sentence which uses every letter of the alphabet at least once.
// Example: "The quick brown fox jumps over the lazy dog."

use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<_>>()
        .len()
        == 26
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn it_works() {
        assert!(is_pangram("the quick brown fox jumps over the lazy dog!"));
        assert!(is_pangram("this is not a pangram!").not());
    }
}

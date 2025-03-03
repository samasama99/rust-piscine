//
// Instructions
//
// Define the function is_permutation, that returns true if the string s1 is a permutation of s2.
//
// s1 is a permutation of s2 if all the elements in s1 appear the same number of times in s2, and all the characters in s1 appear in s2 even if they are in different order.
// Expected Function

use std::collections::HashMap;

fn get_histogram(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
        acc
    })
}
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    s1.len() == s2.len() && get_histogram(s1) == get_histogram(s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";

        assert!(is_permutation(word, word1));
    }
}

//
// Instructions
//
// Define the function is_permutation, that returns true if the string s1 is a permutation of s2.
//
// s1 is a permutation of s2 if all the elements in s1 appear the same number of times in s2, and all the characters in s1 appear in s2 even if they are in different order.
// Expected Function

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    s1.chars().all(|c| s2.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";

        assert_eq!(true, is_permutation(word, word1));
    }
}

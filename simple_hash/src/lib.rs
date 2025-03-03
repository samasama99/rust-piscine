// Instructions
//
// Create a function named word_frequency_counter which will receive a vector of strings
// (each string being a single word)
// and return an HashMap with the word as the key and the number of repetitions as the value.
//
// Also create a function named nb_distinct_words which will take a reference to the HashMap
// and return the number of distinct words present in it.
//
// all the words tested will be lowercase
//
// Expected functions

use std::collections::{HashMap, HashSet};

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    words.into_iter().fold(HashMap::new(), |mut acc, word| {
        acc.entry(word)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
        acc
    })
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count
        .keys()
        .map(|w| w.to_lowercase())
        .fold(HashSet::new(), |mut acc, w| {
            acc.insert(w);
            acc
        })
        .iter()
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{nb_distinct_words, word_frequency_counter};
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(words);

        assert_eq!(
            HashMap::from([
                ("tests", 1),
                ("with", 1),
                ("this", 2),
                ("it", 1),
                ("enough", 1),
                ("is", 2),
                ("but", 1),
                ("sentence", 1),
                ("only", 1),
                ("basic", 3),
                ("again", 1),
                ("for", 1),
                ("be", 1),
                ("once", 1),
                ("very", 2),
                ("should", 1),
                ("few", 1),
                ("a", 2),
                ("repetitions.", 1)
            ]),
            frequency_count
        );
        assert_eq!(20, nb_distinct_words(&frequency_count));
    }
}

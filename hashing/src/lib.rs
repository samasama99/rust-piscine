// Instructions
//
// Given a list of integers write three functions.
//
//     mean: that calculates the mean (the average value) of all the values in the list.
//
//     median: that calculates the median (for a sorted list, it is the value in the middle).
//     If there is an even amount of numbers in the list, the middle pair must be determined,
//     added together, and divided by two to find the median value.
//
//     mode that calculates the mode (the value that appears more often).
//
// Expected Functions

use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.iter().len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    match list.len() % 2 {
        1 => {
            let mut list = list.to_vec();
            list.sort();
            list[list.len() / 2]
        }
        0 => {
            let mut list = list.to_vec();
            list.sort();
            (list[list.len() / 2 - 1] + list[list.len() / 2]) / 2
        }
        _ => unreachable!(),
    }
}

pub fn mode(list: &[i32]) -> i32 {
    *list
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<i32, i32>, &n| {
            acc.entry(n).and_modify(|count| *count += 1).or_insert(1);
            acc
        })
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [4, 7, 5, 2, 5, 1, 3];

        assert_eq!(3.857142857142857, mean(&v));
        assert_eq!(4, median(&v));
        assert_eq!(5, mode(&v));
    }
}

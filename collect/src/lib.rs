// Instructions
//
// Implement the function bubble_sort, which receives a list of integers and sorts it in increasing order using the bubble sort algorithm.
// Expected Function

use std::i32;

pub fn bubble_sort(arr: &mut [i32]) {
    let end = arr.len() - 1;
    for i in 0..arr.len() {
        let current = end - i;
        let (index, max) = {
            let mut index = 0;
            let mut max = i32::MIN;
            for j in 0..=current {
                if arr[j] > max {
                    index = j;
                    max = arr[j];
                }
            }
            (index, max)
        };
        let tmp = arr[current];
        arr[current] = max;
        arr[index] = tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = [3, 2, 4, 5, 1, 7];
        let mut v_clone = v;

        assert_eq!(bubble_sort(&mut v), v_clone.sort_unstable());
    }
}

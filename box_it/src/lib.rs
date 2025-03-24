// Instructions
//
// Create the following functions:
//
// transform_and_save_on_heap: which accepts a string of numbers separated by spaces.
//      If a number has a 'k' as a suffix it should be multiplied by 1000.
//      The function transforms those numbers into a vector of u32, and saves them in the heap using Box.
//
// take_value_ownership: which accepts the return value from transform_and_save_on_heap, unboxes the value, and returns it.

// Expected Functions

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    Box::new(
        s.split(' ')
            .map(|n| {
                if n.ends_with('k') {
                    (n.strip_suffix('k').unwrap().parse::<f64>().unwrap() * 1000.0) as u32
                } else {
                    n.parse().unwrap()
                }
            })
            .collect(),
    )
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");

        let a_h = transform_and_save_on_heap(new_str);
        assert_eq!([5500, 8900, 32], a_h.as_slice());
        assert_eq!(8, size_of_val(&a_h));

        let a_b_v = take_value_ownership(a_h);
        assert_eq!([5500, 8900, 32], a_b_v.as_slice());
        assert_eq!(24, size_of_val(&a_b_v));
    }
}

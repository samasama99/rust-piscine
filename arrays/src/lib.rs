// Define a function named thirtytwo_tens that returns an array with 32 positions filled with only the value 10,
// so that [10, 10, 10, ... 10].len() is equal to 32.

// Write a function that takes an array of i32 and returns the sum of the elements.

pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = (1..=10).collect::<Vec<_>>();
        let b = [5; 10];

        assert_eq!(55, sum(&a));
        assert_eq!(50, sum(&b));
        assert_eq!(
            [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
            thirtytwo_tens()
        );
        assert_eq!(
            32,
            thirtytwo_tens().len()
        );
    }
}

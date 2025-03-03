use std::collections::HashMap;

// Instructions
//
// Create a function named bigger that gets the biggest positive number in the HashMap.
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let hash = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);

        assert_eq!(334, bigger(hash));
    }
}

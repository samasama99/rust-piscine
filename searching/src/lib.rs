// Instructions
//
// Complete the function search. It should return the index of the element which matches key in the array.
//
//     Only arrays with unique elements will be tested.
//
// Expected functions

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        let f = search(&ar, 6);
        assert_eq!(6, f.unwrap());
    }
}

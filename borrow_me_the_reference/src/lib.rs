/*
 *
 * Understanding ownership is essential to take full advantage of Rust's capabilities, as it influences almost all aspects of the language.
 *
 * Create the following functions:
 *
 *     delete_and_backspace: which receives a borrowed string, and processes it.
*     - represents the backspace key and + represents the delete key, so that "helll-o" and "he+lllo" are both converted to "hello".
*       The - and + characters should be removed from the string.
 *
 *     do_operations: which borrows a vector of string literals representing simple addition and subtraction equations. The function should replace the operation with the result.
 *
*/

use std::char;

pub fn delete_and_backspace(s: &mut String) {
    let chars = s.chars();
    let mut stack: Vec<char> = Vec::new();
    let mut final_stack: Vec<char> = Vec::new();

    for c in chars {
        if c == '-' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }    

    for c in stack.into_iter().rev() {
        if c == '+' {
            final_stack.pop();
        } else {
            final_stack.push(c);
        }
    }

    let res : String = final_stack.into_iter().rev().collect();

    s.replace_range(.., &res);
}

pub fn do_operations(v: &mut [String]) {
    let len = v.len();
    for i in 0..len {
        if v[i].contains('+') {
            let res = v[i].split_once('+').unwrap();
            v[i] = (res.0.parse::<i32>().unwrap() + res.1.parse::<i32>().unwrap()).to_string();
        } else {
            let res = v[i].split_once('-').unwrap();
            v[i] = (res.0.parse::<i32>().unwrap() - res.1.parse::<i32>().unwrap()).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];

        delete_and_backspace(&mut a);
        do_operations(&mut b);

        assert_eq!(
            (a, b),
            (
                "borrow".to_owned(),
                [
                    "4".to_owned(),
                    "5".to_owned(),
                    "7".to_owned(),
                    "10".to_owned()
                ]
                .to_owned()
            )
        );
    }
}

// Instructions
//
// Create a function named edit_distance,
//      which calculates the minimum number of changes (insertions, deletions and/or substitutions) which are needed to transform the source string to the target string.

use std::{iter::Peekable, str::Chars, usize};

fn helper(mut source: Peekable<Chars<'_>>, mut target: Peekable<Chars<'_>>) -> usize {
    let source_first_path = source.clone();
    let source_second_path = source.clone();
    let mut target_first_path = target.clone();
    let mut target_second_path = target.clone();

    match (source.peek(), target.peek()) {
        (None, None) => 0,
        (Some(a), Some(b)) if a == b => {
            source.next();
            target.next();
            helper(source, target)
        }
        (Some(a), Some(b)) if a != b => {
            target_first_path.next();
            *target_second_path.peek_mut().unwrap() = *a;
            1 + usize::min(
                helper(source_first_path, target_first_path),
                helper(source_second_path, target_second_path),
            )
        }
        (Some(_), None) => source.count(),
        (None, Some(_)) => target.count(),
        _ => unreachable!()
    }
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let source = source.chars().peekable();
    let target = target.chars().peekable();

    helper(source, target)
}
// Recursive
// one should delete and one should replace, it will alternate between them recursivly and so in case:
// abc
// axxbc
//
// will get
// axxbc abxbc abcbc abc
//                   abc
//             abbc  abcc
//                   abc
//       axbc  abbc  abcc
//                   abc
//             abc
// the last one is the smallest so that will be the number of changes
//
// in case there is smaller number of characters
//
// abc
// xx   axx .... (normal algo)
//      ax  acx ... (normal algo)
//          ab  abc

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_change() {
        let source = "a";
        let target = "a";
        assert_eq!(edit_distance(source, target), 0);
    }
    #[test]
    fn no_change_2() {
        let source = "abc";
        let target = "abc";
        assert_eq!(edit_distance(source, target), 0);
    }
    #[test]
    fn single_char_change() {
        let source = "a";
        let target = "b";
        assert_eq!(edit_distance(source, target), 1);
    }

    #[test]
    fn multi_char_change() {
        let source = "alignment";
        let target = "assignment";

        assert_eq!(edit_distance(source, target), 2);
    }
}

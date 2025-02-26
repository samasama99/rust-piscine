// Sometimes it is more desirable to catch the failure of some parts of a program instead of just calling panic.
//
// For this exercise you will have to create a message blocker, where you must block the word stupid.
//
// You will have to create a structure called Message, which contains:
//
//     elements:
//         content: String
//         user: String
//     associated functions:
//         new: which initializes the structure.
//         send_ms: which only has its implementation type (self) as argument.
//              It should return None if the content of the message is either empty or contains the word stupid. It should return the content of the message otherwise.
//
// You will also need to create a function named check_ms which accepts a reference to a Message, and returns a tuple. This function will invoke the send_ms function.
//
//     If send_ms returns None, then your function should return false and "ERROR: illegal".
//     Else, your function should return true and the contents of the message sent.
//

use std::ops::Not;

pub struct Message {
    pub content: String,
    pub user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Self {
        Self {
            content: ms,
            user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() {
            None
        } else {
            self.content
                .to_lowercase()
                .contains("stupid")
                .not()
                .then_some(self.content.as_ref())
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(message) => (true, message),
        None => (false, "ERROR: illegal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let m0 = Message::new("hello there".to_string(), "toby".to_string());
        assert_eq!((true, "hello there"), check_ms(&m0));
    }
    #[test]
    fn t2() {
        let m1 = Message::new("".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m1));
    }
    #[test]
    fn t3() {
        let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m2));
    }
    #[test]
    fn t4() {
        let m3 = Message::new("stupid".to_string(), "toby".to_string());
        assert_eq!((false, "ERROR: illegal"), check_ms(&m3));
    }
}

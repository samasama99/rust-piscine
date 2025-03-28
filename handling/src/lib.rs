// Create a function named open_or_create which has two arguments:
//
//     file : &str: which represents a file path.
//     content: &str which will be the content to be written to the file.
//
// This function should try to open a file. If it does not exist, the file should be created. In case something goes wrong, it should panic, with the error.

use std::fs::File;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    File::create(file)
        .unwrap()
        .write(content.as_bytes())
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn it_works() {
        let path = "a.txt";
        File::create(path).unwrap();
        open_or_create(path, "content to be written");

        let mut file = File::open(path).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        assert_eq!("content to be written", s);
    }
}

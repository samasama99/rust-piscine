// Create a function that tries to open a file and panics if the file does not exist.

use core::panic;
use std::fs;
use std::fs::File;

pub fn open_file(s: &str) -> File {
    match fs::exists(s) {
        Err(_) | Ok(false) => panic!("File not found"),
        _ => File::open(s).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let filename = "created.txt.test_work";
        File::create(filename).unwrap();

        let a = open_file(filename);
        let full_path = fs::canonicalize(filename);
        println!("{:?}", a);
        assert_eq!(
            "File { fd: 3, path: \"".to_owned() + full_path.unwrap().to_str().unwrap() + "\", read: true, write: false }",
            format!("{:?}", a)
        );
    }

    #[test]
    #[should_panic(expected = "File not found")]
    fn name() {
        let filename = "created.txt.test_panic";
        File::create(filename).unwrap();
        fs::remove_file(filename).unwrap();
        //It must panic
        open_file(filename);
    }
}

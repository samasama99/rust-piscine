// Instructions
//
// The Atbash cipher is an encryption method in which each letter of a word is replaced by its mirror letter in the alphabet.
//
// Your objective is to create a function named cipher which must return a Result wrapped in an Option.
// The Result should contain either a boolean or an Error based on the CipherError structure. This structure should be the error type for the function cipher.
//
// cipher should compare the original String with the ciphered String.
// It should return true if the cipher is correct.
// If the cipher is incorrect it should return the error type CipherError with a boolean and the expected atbash cipher String.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

fn cipher_char(c: char) -> char {
    match c {
        'A'..='Z' => ('Z' as u8 - (c as u8 - 'A' as u8)) as char,
        'a'..='z' => ('z' as u8 - (c as u8 - 'a' as u8)) as char,
        _ => c,
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.trim().is_empty() {
        return None;
    }
    let valid_cipher = original.chars().map(cipher_char).collect::<String>();

    if valid_cipher == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError {
            validation: false,
            expected: valid_cipher,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Some(Ok(true)), cipher("1Hello 2world!", "1Svool 2dliow!"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            Some(Err(CipherError {
                validation: false,
                expected: "1Svool 2dliow!".to_owned()
            })),
            cipher("1Hello 2world!", "svool")
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(None, cipher("", "svool"));
    }
}

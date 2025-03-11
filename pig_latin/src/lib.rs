const VOWELS: &[char] = &['a', 'e', 'u', 'i', 'o'];

pub fn pig_latin(text: &str) -> String {
    let mut chars = text.chars().peekable();
    match chars.peek() {
        None => String::new(),
        Some(v) if VOWELS.contains(v) => text.to_owned() + "ay",
        Some(_) => {
            if let Some((first, second)) = text.split_once("qu") {
                if first.len() == 1 {
                    return second.to_owned() + first + "qu" + "ay";
                }
            }

            let first_vowel = text.find(VOWELS);
            if let Some(first_vowel) = first_vowel {
                let (first, last) = text.split_at(first_vowel + 1);
                let (p1, p2) = first.chars().partition::<Vec<_>, _>(|c| VOWELS.contains(c));
                p1.iter().collect::<String>() + last + p2.iter().collect::<String>().as_ref() + "ay"
            } else {
                text.to_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pig_latin;

    #[test]
    fn it_works() {
        assert_eq!("iglooay", pig_latin(&String::from("igloo")));
        assert_eq!("appleay", pig_latin(&String::from("apple")));
        assert_eq!("ellohay", pig_latin(&String::from("hello")));
        assert_eq!("aresquay", pig_latin(&String::from("square")));
        assert_eq!("enonxay", pig_latin(&String::from("xenon")));
        assert_eq!("airchay", pig_latin(&String::from("chair")));
        assert_eq!("ueenqay", pig_latin(&String::from("queen")));
    }
}

// Instructions
//
// Build the function talking which will allow you to talk with your computer.
//
// Its answers will be created by you following the rules below.
//
// It answers "There is no need to yell, calm down!" if you yell at it. For example "LEAVE ME ALONE!". Yelling is when all the letters are capital letters.
// It answers "Sure." if you ask it something without yelling. For example "Is everything ok with you?".
// It answers "Quiet, I am thinking!" if you yell a question at it. FOr example: "HOW ARE YOU?".
// It says "Just say something!" if you address it without actually saying anything.
// It answers "Interesting" to anything else.

pub fn talking(text: &str) -> &str {
    let text = text.trim();
    if text.is_empty() {
        "Just say something!"
    } else if text
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
    {
        if text.ends_with("!") {
            "There is no need to yell, calm down!"
        } else {
            "Quiet, I am thinking!"
        }
    } else if text.ends_with("?") {
        "Sure."
    } else {
        "Interesting"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "There is no need to yell, calm down!",
            talking("JUST DO IT!")
        );
        assert_eq!("Sure.", talking("Hello how are you?"));
        assert_eq!("Quiet, I am thinking!", talking("WHAT'S GOING ON?"));
        assert_eq!("Interesting", talking("something"));
        assert_eq!("Just say something!", talking(""));
    }
}

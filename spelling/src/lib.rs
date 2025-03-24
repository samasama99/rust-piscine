pub fn spell(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16..=17 => format!("{}teen", spell(n % 10)),
        18 => "eighteen".to_string(),
        19 => format!("{}-teen", spell(n % 10)),
        20 => "twenty".to_string(),
        21..=29 => format!("twenty-{}", spell(n % 10)),
        30 => "thirty".to_string(),
        30..=39 => format!("thirty-{}", spell(n % 10)),
        40 => "forty".to_string(),
        41..=49 => format!("forty-{}", spell(n % 10)),
        50 => "fifty".to_string(),
        51..=59 => format!("fifty-{}", spell(n % 10)),
        60 => "sixty".to_string(),
        61..=69 => format!("sixty-{}", spell(n % 10)),
        70 => "seventy".to_string(),
        71..=79 => format!("seventy-{}", spell(n % 10)),
        80 => "seventy".to_string(),
        81..=89 => format!("eighty-{}", spell(n % 10)),
        90 => "ninety".to_string(),
        91..=99 => format!("ninety-{}", spell(n % 10)),
        100 => "one hundred".to_string(),
        101..=999 => format!("{} hundred {}", spell(n / 100), spell(n % 100)).to_string(),
        1000 => "one thousand".to_string(),
        1001..=999999 => {
            format!(
                "{}{}{}",
                if (n / 1000) != 0 {
                    format!("{} thousand ", spell(n / 1000))
                } else {
                    String::from("")
                },
                if (n / 100 % 10) != 0 {
                    format!("{} hundred ", spell(n / 100 % 10),)
                } else {
                    String::from("")
                },
                if (n % 100) != 0 {
                    spell(n % 100)
                } else {
                    String::from("")
                }
            ).trim().to_string()
        }
        1_000_000 => "one million".to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("zero", spell(0));
        assert_eq!("eleven", spell(11));
        assert_eq!("three hundred forty-eight", spell(348));
        assert_eq!("one thousand fifty-five", spell(1055));
        assert_eq!("nine thousand nine hundred ninety-six", spell(9996));
        assert_eq!(
            "six hundred fifty thousand one hundred twenty-three",
            spell(650_123)
        );
        assert_eq!("eight hundred ten thousand", spell(810_000));
    }
}

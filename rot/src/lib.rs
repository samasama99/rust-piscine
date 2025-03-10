pub fn rotate(input: &str, key: i8) -> String {
    let key = key % 26;
    let key: u8 = if key < 0 { (key + 26) as u8 } else { key as u8 };
    input
        .chars()
        .map(|c| {
            if !c.is_alphabetic() {
                c
            } else if c.is_ascii_uppercase() {
                let c = c as u8 - 'A' as u8;
                let c = (c + key) % 26;
                (c + 'A' as u8) as char
            } else {
                let c = c as u8 - 'a' as u8;
                let c = (c + key) % 26;
                (c + 'a' as u8) as char
            }
        })
        .collect::<String>()
}

#[test]
fn test() {
    assert_eq!("a", rotate("a", 26));
    assert_eq!("m", rotate("m", 0));
    assert_eq!("z", rotate("m", 13));
    assert_eq!("p", rotate("a", 15));
    assert_eq!(
        "The five boxing wizards jump quickly.",
        rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13)
    );
}

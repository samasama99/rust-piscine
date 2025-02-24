pub fn arrange_phrase(phrase: &str) -> String {
    let splited = phrase.split(" ");
    let mut v: Vec<(u32, String)> = splited
        .map(|s| (s.find(|c: char| c.is_digit(10)).unwrap(), s.to_owned()))
        .map(|(index, mut s)| (s.remove(index).to_digit(10).unwrap(), s))
        .collect();
    v.sort_by_key(|&(index, _)| index);
    v.into_iter().map(|(_, s)| s).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("This is a Test", arrange_phrase("is2 Thi1s T4est 3a"));
    }
}

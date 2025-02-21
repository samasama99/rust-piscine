const DOUBT: char = '?';

pub fn doubtful(s: &mut String) {
    s.push(DOUBT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "Hello".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }
}

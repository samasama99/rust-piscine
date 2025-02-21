pub fn to_url(s: &str) -> String {
    s.to_owned().replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        assert_eq!(to_url(s), "Hello,%20world!");
    }
}

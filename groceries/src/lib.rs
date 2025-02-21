pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    slice.get(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut groceries = vec!["yogurt".to_string(), "panettone".to_string()];

        let groceries_after_insert = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "nuts".to_string(),
        ];

        insert(&mut groceries, String::from("nuts"));
        assert_eq!(groceries, groceries_after_insert);
        assert_eq!(at_index(&groceries, 1), "panettone");
    }
}

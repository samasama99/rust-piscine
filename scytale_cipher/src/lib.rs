use std::ops::Not;

pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut message = message;
    let mut chunks: Vec<Vec<_>> = Vec::new();
    for c in message.chars() {
        if let Some(last) = chunks.last_mut() {
            if last.len() < i as usize {
                last.push(c);
            } else {
                chunks.push(vec![c]);
            }
        } else {
            chunks.push(vec![c]);
        }
    }

    if let Some(last) = chunks.last_mut() {
        if last.len() < i as usize {
            last.extend(vec![' '; i as usize - last.len()]);
            message.extend(vec![' '; i as usize - last.len()]);
        }
    }

    let mut not_empty = true;
    let mut res = vec![];
    while not_empty {
        not_empty = false;
        for chunk in &mut chunks {
            if chunk.is_empty().not() {
                not_empty = true;
                res.push(chunk.remove(0));
            }
        }
    }

    res.iter().collect::<String>().trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::scytale_cipher;

    #[test]
    fn it_works() {
        assert_eq!(
            "sec yCtoadle",
            scytale_cipher(String::from("scytale Code"), 6)
        );
        assert_eq!(
            "sCcoydtea l e",
            scytale_cipher(String::from("scytale Code"), 8)
        );
    }
}

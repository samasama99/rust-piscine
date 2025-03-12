fn offsets(c: char, target: char) -> (String, String) {
    if c == 'A' {
        (" ".repeat(target as usize - c as usize), String::new())
    } else {
        (
            " ".repeat(target as usize - c as usize),
            " ".repeat((c as usize - 'A' as usize) * 2 - 1),
        )
    }
}

pub fn get_diamond(target: char) -> Vec<String> {
    let target = target.to_ascii_uppercase();

    ('A'..=target)
        .chain(('A'..target).rev())
        .map(|c| (c.to_string(), offsets(c, target)))
        .map(|(c, (sides, middle))| {
            if middle.is_empty() {
                format!("{sides}{c}{sides}")
            } else {
                format!("{sides}{c}{middle}{c}{sides}")
            }
        })
        .collect()
}

fn offset(c: char, start: char, end: char) -> (String, String) {
    (
        " ".repeat(end as usize - c as usize),
        " ".repeat((c as usize - start as usize) * 2 - 1),
    )
}

pub fn get_diamond(target: char) -> Vec<String> {
    let target = target.to_ascii_uppercase();
    let mut res = vec![];
    let first_sides = " ".repeat(target as usize - 'A' as usize);
    res.push(first_sides.to_owned() + "A" + &first_sides);
    res.extend(
        ('B'..=target).chain(('B'..target).rev())
            .map(|c| (c, offset(c, 'A', target)))
            .map(|(c, (sides, middle))| {
                sides.to_owned() + &c.to_string() + &middle.to_string() + &c.to_string() + &sides
            }),
    );
    res.push(first_sides.to_owned() + "A" + &first_sides);
    res
}

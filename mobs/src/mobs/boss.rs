// boss: which should contain:
//
// Boss: a struct which consists of:
// name: String
// age: u8
// new: an associated function which accepts a name and age, and returns a Boss.
#[derive(Debug)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    pub fn new(name: &str, age: u8) -> Self {
        Boss {
            name: name.to_string(),
            age,
        }
    }
}

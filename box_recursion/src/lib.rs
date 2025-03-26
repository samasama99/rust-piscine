// Instructions

// Using the given code, create the following associated functions:

// new: which will initialize the WorkEnvironment with grade set to None.
// add_worker: which receives two strings, one being the role and the other the name of the worker.
//      It will add the worker at the start of the list.
// remove_worker: which removes the last worker that was placed in the WorkEnvironment,
//      this function returns an Option with the name of the worker.
// last_worker: which returns an Option with a tuple containing the name and role of the last added worker.

// You must also create a type named Link. This will be the connection between the WorkEnvironment and Worker structures.
//      This will be a recursion type, and it must point to None if there is no Worker to point to.
// Expected Functions and structures

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        }));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let result = self.grade.take();
        if let Some(result) = result {
            self.grade = result.next;
            Some(result.name)
        } else {
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade
            .as_ref()
            .map(|g| (g.name.to_owned(), g.role.to_owned()))
    }
}

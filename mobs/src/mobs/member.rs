// member submodule which consists of:
// Role: an enum with the variants:
// Underboss
// Caporegime
// Soldier
// Associate

// Member: a struct which consists of:
// name: String
// role: Role
// age: u8
// get_promotion: an associated function which when invoked should promote the member from:
// Associate -> Soldier
// Soldier -> Caporegime
// Caporegime -> Underboss
// new: accepts a name, role and age, returning a Member.

#[derive(Debug)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Member {
            name: name.to_owned(),
            role,
            age,
        }
    }
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => {
                self.role = Role::Soldier;
            }
            Role::Soldier => {
                self.role = Role::Caporegime;
            }
            Role::Caporegime => {
                self.role = Role::Underboss;
            }
            Role::Underboss => {}
        }
    }
}

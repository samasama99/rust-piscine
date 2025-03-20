// pub use crate::boss::Boss;
// pub use crate::member::Member;
pub use crate::mobs::boss;
pub use crate::mobs::boss::Boss;
pub use crate::mobs::member;
pub use crate::mobs::member::Member;

pub mod mobs;

#[derive(Debug)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, i32)>,
    pub members: Vec<Member>,
    pub wealth: i32,
}

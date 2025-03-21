pub use crate::mobs::boss::Boss;
pub use crate::mobs::member::Member;
pub use crate::mobs::member::*;

pub mod boss;
pub mod member;
#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub cities: Vec<(String, i32)>,
    pub members: Vec<Member>,
    pub wealth: i32,
}

// Create a module named mobs, containing a structure Mob which has:
//
//  Name: String
//  Boss: Boss
//  Members: a vector of Member
//  Cities: a vector of tuples containing a city name and a u8
//  Wealth: u32
//  Recruit: an associated function which adds a Member to the members vector.
//    It should accept a name, and an age. The member's role should be set to Associate.
impl Mob {
    pub fn recruit(&mut self, name: String, age: u8) {
        self.members.push(Member {
            name,
            age,
            role: Role::Associate,
        })
    }
}
//  Attack: an associated function which receives another Mob.
//    It will remove the last member from the vector of Member from whichever mob has the least power combat score.
//    In the case of a draw, the attacker loses. In the case that one of the mobs is left with zero members,
//    the victorious mob will also take the cities and wealth from the losing mob. The power combat score is calculated from the sum of the role of each mob member:
//      Underboss: 4
//      Caporegime: 3
//      Soldier: 2
//      Associate: 1

impl Role {
    fn to_power(&self) -> usize {
        match self {
            Role::Associate => 1,
            Role::Soldier => 2,
            Role::Caporegime => 3,
            Role::Underboss => 4,
        }
    }
}
impl Mob {
    pub fn attack(&mut self, mob: &mut Mob) {
        let mob_members_1 = &mut self.members;
        let mob_members_2 = &mut mob.members;
        if mob_members_1.len() == 0 && mob_members_2.len() != 0 {
            mob.wealth += self.wealth;
            mob.cities.append(&mut self.cities);
        } else if mob_members_2.len() == 0 && mob_members_1.len() != 0 {
            self.wealth += mob.wealth;
            self.cities.append(&mut mob.cities);
        }
        let total_1 = mob_members_1
            .iter()
            .map(|member| member.role.to_power())
            .sum::<usize>();
        let total_2 = mob_members_2
            .iter()
            .map(|member| member.role.to_power())
            .sum::<usize>();

        if total_1 > total_2 {
            mob_members_2.pop().unwrap();
        } else {
            mob_members_1.pop().unwrap();
        }
    }
}
//  Steal: an associated function which receives a Mob to target, and a u32 value to steal.
//    The 'self' mob steals the value from the wealth of the target mob, and adds the value to its own wealth. Only as much money as the target mob has can be stolen.
impl Mob {
    pub fn steal(&mut self, mob: &mut Mob, amount: u32) {
        let amount = mob.wealth.min(amount as i32);
        self.wealth += amount;
        mob.wealth -= amount;
    }
}
//  Conquer_city: an associated function which receives a vector of Mob, a city name and a u8 value.
//    The city name and u8 value are added to its list of cities if non of the other mobs in the vector have a city with the same name.
impl Mob {
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city: &str, n: u8) {
        if mobs
            .iter()
            .flat_map(|mob| mob.cities.iter())
            .find(|(city_name, _)| city_name == city)
            .is_none()
        {
            self.cities.push((city.to_string(), n as i32));
        }
    }
}

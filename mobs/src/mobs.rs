pub mod boss;
pub mod member;

pub use crate::mobs::boss::Boss;
pub use crate::mobs::member::Member;
pub use crate::Role;

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
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member {
            name: name.to_string(),
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

        dbg!(&mob_members_1);
        dbg!(&mob_members_2);

        if mob_members_1.len() == 0 && mob_members_2.len() != 0 {
            mob.wealth += self.wealth;
            self.wealth = 0;
            mob.cities.append(&mut self.cities);
        } else if mob_members_2.len() == 0 && mob_members_1.len() != 0 {
            self.wealth += mob.wealth;
            mob.wealth = 0;
            self.cities.append(&mut mob.cities);
        } else {
            return;
        }
        let total_1 = mob_members_1
            .iter()
            .map(|member| member.role.to_power())
            .sum::<usize>();
        let total_2 = mob_members_2
            .iter()
            .map(|member| member.role.to_power())
            .sum::<usize>();

        if total_1 >= total_2 {
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
    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city: String, n: u8) {
        if mobs
            .iter()
            .flat_map(|mob| mob.cities.iter())
            .find(|(city_name, _)| city_name == &city)
            .is_none()
        {
            self.cities.push((city, n as i32));
        }
    }
}

// Compiling mobs v0.1.0 (/jail/solutions/mobs)
// Compiling mobs_test v0.1.0 (/jail/tests/mobs_test)
// Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s
// Running unittests src/main.rs (tests/mobs_test/target/debug/deps/mobs_test-86ff66d5c255f062)
//
// running 8 tests
// test test::create_boss_and_members ... ok
// test test::member_get_promotion ... ok
// test test::mob_conquer_city ... ok
// test test::mob_attack ... FAILED
// test test::mob_recruit ... ok
// test test::mob_steal ... ok
// test test::no_members_mob ... FAILED
// test test::same_combat_power ... FAILED
//
// failures:
//
// ---- test::mob_attack stdout ----
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// thread 'test::mob_attack' panicked at src/main.rs:119:9:
// assertion `left == right` failed
// error: test failed, to rerun pass `--bin mobs_test`
// left: 4
// right: 3
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// ---- test::no_members_mob stdout ----
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// ]
// thread 'test::no_members_mob' panicked at src/main.rs:151:9:
// assertion `left == right` failed
// left: 4
// right: 0
//
// ---- test::same_combat_power stdout ----
// [/jail/solutions/mobs/src/mobs.rs:59:9] &mob_members_1 = [
// Member {
// name: "Benny Eggs",
// role: Soldier,
// age: 28,
// },
// Member {
// name: "Jhonny",
// role: Associate,
// age: 17,
// },
// Member {
// name: "Greasy Thumb",
// role: Soldier,
// age: 30,
// },
// Member {
// name: "No Finger",
// role: Caporegime,
// age: 32,
// },
// Member {
// name: "Stitches",
// role: Associate,
// age: 28,
// },
// ]
// [/jail/solutions/mobs/src/mobs.rs:60:9] &mob_members_2 = [
// Member {
// name: "Knuckles",
// role: Soldier,
// age: 25,
// },
// Member {
// name: "Baldy Dom",
// role: Caporegime,
// age: 36,
// },
// Member {
// name: "Crazy Joe",
// role: Underboss,
// age: 23,
// },
// ]
// thread 'test::same_combat_power' panicked at src/main.rs:139:9:
// assertion `left == right` failed
// left: 5
// right: 4
//
//
// failures:
// test::mob_attack
// test::no_members_mob
// test::same_combat_power
//
// test result: FAILED. 5 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
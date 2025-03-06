// Instructions
//
// Using the mall module provided, create the following functions to help run a shopping mall:
//
//     biggest_store: receives a Mall and returns the Store with the biggest square_meters.
//     highest_paid_employee: receives a Mall and returns a vector containing the Employee(s) with the highest salary.
//     nbr_of_employees: receives a Mall and returns the number of employees and guards as an usize.
//     check_for_securities: receives a Mall and a vector of Guard.
//                  If there is not at least 1 guard for every 200 square meters of floor size,
//                  a guard should be added to the Mall.guards.
//     cut_or_raise: receives a Mall. For each employee, the salary will be raised by 10% if they work more than 10 hours,
//                  else their salary will be decreased by 10%. You can consider that guards are not employees of the mall.

// pub mod mall;
pub use crate::mall::floor::store;
pub use crate::mall::floor::store::employee::Employee;
pub use crate::mall::floor::store::Store;
pub use crate::mall::guard::Guard;
pub use crate::mall::{floor, Mall};

pub fn cut_or_raise(mall: &mut Mall) {
    let (mut need_a_raise, mut nope): (Vec<&mut Employee>, Vec<&mut Employee>) = mall
        .floors
        .iter_mut()
        .flat_map(|floor| floor.stores.iter_mut().flat_map(|store| store.employees.iter_mut()))
        .partition(|employee| {
            employee.working_hours.1 - employee.working_hours.0 >= 10
        });
    need_a_raise
        .iter_mut()
        .for_each(|employee| employee.salary += employee.salary * 0.1);
    nope.iter_mut()
        .for_each(|employee| employee.salary -= employee.salary * 0.1);
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let total: u64 = mall
        .floors
        .iter()
        .map(|floor| {
            floor
                .stores
                .iter()
                .map(|floor| floor.square_meters)
                .sum::<u64>()
        })
        .sum();
    let guards_len = mall.guards.len() as u64;

    let guard_for = total / guards_len / 200;
    let guard_for = guard_for.min(guards.len() as u64);
    for i in 0..guard_for {
        mall.guards.push(guards[i as usize].clone());
    }
}

pub fn highest_paid_employee(mall: Mall) -> Employee {
    mall.floors
        .iter()
        .flat_map(|floor| {
            floor
                .stores
                .iter()
                .flat_map(|stores| stores.employees.clone())
        })
        .max_by_key(|employee: &Employee| employee.salary.round() as usize)
        .unwrap()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .map(|floor| floor.stores.clone())
            .map(|stores| {
                stores
                    .iter()
                    .map(|store| store.employees.len())
                    .sum::<usize>()
            })
            .sum::<usize>()
}

pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.clone())
        .max_by_key(|store: &Store| store.square_meters)
        .unwrap()
}

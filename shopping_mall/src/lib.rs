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

pub use crate::mall::Mall;
pub mod mall;
pub use mall::floor::store::employee::*;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::guard::*;
pub use mall::*;

pub fn cut_or_raise(mall: &mut Mall) {
    let (mut employees_to_get_raise, mut employees_to_get_cut): _ = mall
        .floors
        .iter_mut()
        .flat_map(|floor| floor.stores.iter_mut())
        .flat_map(|store| store.employees.iter_mut())
        .partition::<Vec<_>, _>(|employee| {
            employee.working_hours.1 - employee.working_hours.0 >= 10
        });

    employees_to_get_raise
        .iter_mut()
        .for_each(|e| e.salary += e.salary * 0.1);

    employees_to_get_cut
        .iter_mut()
        .for_each(|e| e.salary -= e.salary * 0.1);
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let total_surface = mall.floors.iter().map(|f| f.size_limit).sum::<u64>();

    let needed_guards = total_surface as usize / mall.guards.len() / 200;

    mall.guards.extend(guards.into_iter().take(needed_guards));
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let employees = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|store| store.employees.iter());

    let max = employees
        .clone()
        .map(|employee| employee.salary.round() as usize)
        .max()
        .expect("No employees found");

    employees
        .filter(|employee| employee.salary.round() as usize == max)
        .map(|employee| employee.clone())
        .collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .flat_map(|floor| floor.stores.iter())
            .map(|store| store.employees.len())
            .sum::<usize>()
}

pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|store| store.square_meters)
        .expect("No store found")
        .clone()
}

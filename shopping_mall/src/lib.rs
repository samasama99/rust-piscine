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

pub mod mall;
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


#[allow(dead_code)]
fn main() {
    let secs = vec![
        mall::guard::Guard::new("John Oliver", 34, 7),
        mall::guard::Guard::new("Logan West", 23, 2),
        mall::guard::Guard::new("Bob Schumacher", 53, 15),
    ];

    let footzo_emp = vec![
        mall::floor::store::employee::Employee::new("Finbar Haines", 36, 9, 14, 650.88),
        mall::floor::store::employee::Employee::new("Roksanna Rocha", 45, 13, 22, 772.00),
        mall::floor::store::employee::Employee::new("Sienna-Rose Penn", 26, 9, 22, 1000.43),
    ];
    let swashion_emp = vec![
        mall::floor::store::employee::Employee::new("Abdallah Stafford", 54, 8, 22, 1234.21),
        mall::floor::store::employee::Employee::new("Marian Snyder", 21, 8, 14, 831.90),
        mall::floor::store::employee::Employee::new("Amanda Mclean", 29, 13, 22, 1222.12),
        mall::floor::store::employee::Employee::new("Faizaan Castro", 32, 11, 18, 1106.43),
    ];
    let pizbite_emp = vec![
        mall::floor::store::employee::Employee::new("Juniper Cannon", 21, 16, 23, 804.35),
        mall::floor::store::employee::Employee::new("Alena Simon", 28, 9, 15, 973.54),
        mall::floor::store::employee::Employee::new("Yasemin Collins", 29, 9, 19, 986.33),
        mall::floor::store::employee::Employee::new("Areeb Roberson", 54, 9, 22, 957.82),
        mall::floor::store::employee::Employee::new("Rocco Amin", 44, 13, 23, 689.21),
    ];
    let grill_emp = vec![
        mall::floor::store::employee::Employee::new("Rhian Crowther", 45, 9, 15, 841.18),
        mall::floor::store::employee::Employee::new("Nikkita Steadman", 52, 14, 22, 858.61),
        mall::floor::store::employee::Employee::new("Reginald Poole", 32, 9, 22, 1197.64),
        mall::floor::store::employee::Employee::new("Minnie Bull", 54, 14, 22, 1229.73),
    ];
    let sumo_emp = vec![
        mall::floor::store::employee::Employee::new("Chantelle Barajas", 20, 8, 22, 969.22),
        mall::floor::store::employee::Employee::new("Hywel Rudd", 49, 12, 22, 695.74),
        mall::floor::store::employee::Employee::new("Marianne Beasley", 55, 8, 14, 767.83),
    ];
    let supermaket_emp = vec![
        mall::floor::store::employee::Employee::new("Amara Schaefer", 23, 9, 14, 796.21),
        mall::floor::store::employee::Employee::new("Yara Wickens", 39, 9, 14, 853.42),
        mall::floor::store::employee::Employee::new("Tomi Boyer", 64, 9, 14, 881.83),
        mall::floor::store::employee::Employee::new("Greta Dickson", 42, 9, 14, 775.10),
        mall::floor::store::employee::Employee::new("Caroline Finnegan", 41, 9, 14, 702.92),
        mall::floor::store::employee::Employee::new("Indiana Baxter", 33, 13, 20, 991.71),
        mall::floor::store::employee::Employee::new("Jadine Page", 48, 13, 20, 743.21),
        mall::floor::store::employee::Employee::new("Husna Ryan", 43, 13, 20, 655.75),
        mall::floor::store::employee::Employee::new("Tyler Hunt", 63, 13, 20, 668.25),
        mall::floor::store::employee::Employee::new("Dahlia Caldwell", 56, 13, 20, 781.38),
        mall::floor::store::employee::Employee::new("Chandler Mansell", 20, 19, 24, 656.75),
        mall::floor::store::employee::Employee::new("Mohsin Mcgee", 30, 19, 24, 703.83),
        mall::floor::store::employee::Employee::new("Antoine Goulding", 45, 19, 24, 697.12),
        mall::floor::store::employee::Employee::new("Mark Barnard", 53, 19, 24, 788.81),
    ];

    let ground_stores = vec![
        store::Store::new("Footzo", 50, footzo_emp),
        store::Store::new("Swashion", 43, swashion_emp),
    ];
    let food_stores = vec![
        store::Store::new("PizBite", 60, pizbite_emp),
        store::Store::new("Chillout Grill", 50, grill_emp),
        store::Store::new("Sumo Food", 30, sumo_emp),
    ];
    let supermarket = vec![store::Store::new("Pretail", 950, supermaket_emp)];

    let floors = vec![
        floor::Floor::new("Ground Floor", ground_stores, 300),
        floor::Floor::new("Food Floor", food_stores, 500),
        floor::Floor::new("Supermarket", supermarket, 1000),
    ];

    let mut mall_la_vie = mall::Mall::new("La Vie Funchal", secs, floors);

    //returns the biggest store
    println!("Biggest store: {:#?}", biggest_store(mall_la_vie.clone()));

    //returns the list with the highest paid employees
    println!(
        "Highest paid employee: {:#?}",
        highest_paid_employee(mall_la_vie.clone())
    );

    //returns the number of employees
    println!(
        "Number of employees: {:?}",
        nbr_of_employees(mall_la_vie.clone())
    );

    //checks if it is needed to add securities
    check_for_securities(
        &mut mall_la_vie,
        vec![
            mall::guard::Guard::new("Peter Solomons", 45, 20),
            mall::guard::Guard::new("William Charles", 32, 10),
            mall::guard::Guard::new("Leonardo Changretta", 23, 0),
            mall::guard::Guard::new("Vlad Levi", 38, 8),
            mall::guard::Guard::new("Faruk Berkai", 40, 15),
            mall::guard::Guard::new("Chritopher Smith", 35, 9),
            mall::guard::Guard::new("Jason Mackie", 26, 2),
            mall::guard::Guard::new("Kenzie Mair", 34, 8),
            mall::guard::Guard::new("Bentley Larson", 33, 10),
            mall::guard::Guard::new("Ray Storey", 37, 12),
        ],
    );

    //raises or cuts the salary  of every employee
    cut_or_raise(&mut mall_la_vie);

    println!("{:#?}", &mall_la_vie);
}

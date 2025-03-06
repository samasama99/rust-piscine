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

pub use floor::store::{employee::Employee, Store};
pub use guard::Guard;

#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: Vec<guard::Guard>,
    pub floors: Vec<floor::Floor>,
}

impl Mall {
    #[allow(dead_code)]
    pub fn new(name: &str, guards: Vec<guard::Guard>, floors: Vec<floor::Floor>) -> Mall {
        Mall {
            name: name.to_string(),
            guards: guards,
            floors: floors,
        }
    }

    #[allow(dead_code)]
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    #[allow(dead_code)]
    pub fn hire_guard(&mut self, guard: guard::Guard) {
        self.guards.push(guard);
    }

    #[allow(dead_code)]
    pub fn fire_guard(&mut self, name: String) {
        self.guards.retain(|x| x.name != name);
    }
}

pub mod guard {

    #[derive(Debug, Clone, PartialEq)]
    pub struct Guard {
        pub name: String,
        pub age: u8,
        pub years_experience: u8,
    }

    impl Guard {
        #[allow(dead_code)]
        pub fn new(name: &str, age: u8, years_experience: u8) -> Guard {
            Guard {
                name: name.to_string(),
                age: age,
                years_experience: years_experience,
            }
        }
    }
}

pub mod floor {

    #[derive(Debug, Clone, PartialEq)]
    pub struct Floor {
        pub name: String,
        pub stores: Vec<store::Store>,
        pub size_limit: u64,
    }

    impl Floor {
        #[allow(dead_code)]
        pub fn new(name: &str, stores: Vec<store::Store>, store_limit: u64) -> Floor {
            Floor {
                name: name.to_string(),
                stores: stores,
                size_limit: store_limit,
            }
        }

        #[allow(dead_code)]
        pub fn change_store(&mut self, store: &str, new_store: store::Store) {
            let pos = self.stores.iter().position(|x| x.name == store).unwrap();
            self.stores[pos] = new_store;
        }

        #[allow(dead_code)]
        pub fn add_store(&mut self, new_store: store::Store) {
            let mut current_floor_size = 0;

            for store in self.stores.iter() {
                current_floor_size += store.square_meters;
            }

            if self.size_limit < current_floor_size + new_store.square_meters {
                self.stores.push(new_store);
            }
        }

        #[allow(dead_code)]
        pub fn remove_store(&mut self, store_name: String) {
            self.stores.retain(|x| x.name != store_name);
        }
    }

    pub mod store {

        #[derive(Debug, Clone, PartialEq)]
        pub struct Store {
            pub name: String,
            pub square_meters: u64,
            pub employees: Vec<employee::Employee>,
        }

        impl Store {
            #[allow(dead_code)]
            pub fn new(name: &str, space: u64, employees: Vec<employee::Employee>) -> Store {
                Store {
                    name: name.to_string(),
                    square_meters: space,
                    employees: employees,
                }
            }

            #[allow(dead_code)]
            pub fn hire_employee(&mut self, employee: employee::Employee) {
                self.employees.push(employee);
            }
            #[allow(dead_code)]
            pub fn fire_employee(&mut self, employee_name: &str) {
                self.employees.retain(|x| x.name != employee_name);
            }
            #[allow(dead_code)]
            pub fn expand(&mut self, square_meters: u64) {
                self.square_meters += square_meters;
            }
        }

        pub mod employee {

            #[derive(Debug, Clone, PartialEq)]
            pub struct Employee {
                pub name: String,
                pub age: u8,
                pub working_hours: (u8, u8),
                pub salary: f64,
            }

            impl Employee {
                #[allow(dead_code)]
                pub fn new(
                    name: &str,
                    age: u8,
                    entry_hour: u8,
                    exit_hour: u8,
                    salary: f64,
                ) -> Employee {
                    Employee {
                        name: name.to_string(),
                        age: age,
                        working_hours: (entry_hour, exit_hour),
                        salary: salary,
                    }
                }

                #[allow(dead_code)]
                pub fn birthday(&mut self) {
                    self.age += 1;
                }

                #[allow(dead_code)]
                pub fn change_workload(&mut self, entry_hour: u8, exit_hour: u8) {
                    self.working_hours = (entry_hour, exit_hour);
                }

                #[allow(dead_code)]
                pub fn raise(&mut self, amount: f64) {
                    self.salary += amount;
                }

                #[allow(dead_code)]
                pub fn cut(&mut self, amount: f64) {
                    self.salary = self.salary - amount;
                }
            }
        }
    }
}

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

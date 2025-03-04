// Instructions
//
// The objective is to create an API to parse a list of todos that are organized in a JSON file.
// You must handle all possible errors in a multiple error system.
//
// Organization of the JSON file:
//
// {
//   "title": "TODO LIST FOR PISCINE RUST",
//   "tasks": [
//     { "id": 0, "description": "do this", "level": 0 },
//     { "id": 1, "description": "do that", "level": 5 }
//   ]
// }
//
//
// for lib.rs
//
mod err;

use crate::err::{ParseErr, ReadErr};
use json::JsonValue;
pub use json::{parse, stringify};
pub use std::error::Error;
use std::{fs::File, io::Read};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(path).map_err(|err| ReadErr {
            child_err: err.into(),
        })?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(|err| ReadErr {
            child_err: err.into(),
        })?;
        let res: JsonValue = parse(&buf).map_err(|err| ParseErr::Malformed(err.into()))?;

        let tasks = &res["tasks"];

        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut v = Vec::new();
        for i in 0..tasks.len() {
            v.push(Task {
                id: tasks[i]["id"].as_u32().unwrap(),
                description: tasks[i]["description"].as_str().unwrap().to_string(),
                level: tasks[i]["level"].as_u32().unwrap(),
            });
        }

        Ok(TodoList {
            title: res["title"].as_str().ok_or("")?.to_owned(),
            tasks: v,
        })
    }
}
// lib.rs
//
// In the lib file you will have to implement a function called get_todo which receives a string and returns a Result which can be the structure TodoList or a boxing error. This function must be able to deserialize the json file.
//
// Basically it must parse and read the JSON file and return the TodoList if everything is fine, otherwise it returns the error.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let todos = TodoList::get_todo("todo.json");
        match todos {
            Ok(list) => assert_eq!(
                TodoList {
                    title: "TODO LIST FOR PISCINE RUST".to_string(),
                    tasks: [
                        Task {
                            id: 0,
                            description: "do this".to_string(),
                            level: 0
                        },
                        Task {
                            id: 1,
                            description: "do that".to_string(),
                            level: 5
                        }
                    ]
                    .to_vec()
                },
                list
            ),
            Err(e) => {
                unreachable!();
            }
        }
        let todos = TodoList::get_todo("todo_empty.json");
        dbg!(match todos {
            Ok(list) => unreachable!(),
            Err(e) => {
                assert_eq!(
                    "Fail to parse todo None",
                    format!("{} {:?}", e.to_string(), e.source())
                );
            }
        });
        let todos = TodoList::get_todo("malformed_object.json");
        dbg!(match todos {
            Ok(list) => println!("{:?}", list),
            Err(e) => {
                assert_eq!("Fail to parse todo Some(Malformed(UnexpectedCharacter { ch: '}', line: 4, column: 54 }))",
                    format!("{} {:?}", e.to_string(), e.source()));
            }
        });
    }
}

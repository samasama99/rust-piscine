// err.rs
//
// Create a module in a file named err.rs which handles the boxing of errors.
//
// This module must implement an enum called ParseErr which will take care of the parsing errors. It must have the following elements:
//
//     Empty
//     Malformed: which has a dynamic boxed error as element
//
// A structure called ReadErr which will take care of the reading errors, with an element called child_err of type Box<dyn Error>.
//
// For each data structure, you will have to implement a function called fmt for the Display trait.
//          It should write the message "Fail to parse todo" in the case of any parsing error.
//          Otherwise, it should write the message "Fail to read todo file".
//
// For the Error trait, the following functions (methods) have to be implemented:
//
//     source which returns an Option with the error:
//         For the ReadErr, it must return the option with the error.
//         For the ParseErr, it will return an option which is None if the tasks are empty, and the error if the parsing is malformed.
//
//
// For err.rs
//
use std::error::Error;
use std::fmt;
use std::fmt::{Display};
use crate::err::ParseErr::{Empty, Malformed};

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to parse todo")
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Empty  => None,
            Malformed(_) => Some(self),
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

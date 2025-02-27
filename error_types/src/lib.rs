// Instructions
//
// For this exercise, you will have to implement an error type for a form validator. This must validate the password and the first name.
//
// The first name must not be empty and the password must have at least 8 characters, and a combination of alphabetic, numeric and none-alphanumeric (<, &, / ...).
//
// Examples:
//
// "asDd123=%": good.
// "asgfD": error as it only contains alphabetic characters.
// "asdsdf2": error as it is missing none-alphanumeric characters.
// "sad\_#$": error as it is missing numeric characters.
//
// Create a structure named Form that will have the following fields:
//
// first_name: String
// last_name: String
// birth: NaiveDate that will convert a string "2015-09-05" to a date of that format.
// birth_location: String
// password: String
//
// You must implement the associated functions new and validate that will validate the form.
//
// For the error type you must create a struct named FormError. It must have the fields:
//
// form_values: this will be a tuple of strings representing the invalid input. For example: ("password", "asdaSD\_") or ("first_name", "someone")
//
// date: that will have the date that the error occurred in the format "2020-12-14 09:33:41"
//
// err: the error description:
// "No user name"
// "At least 8 characters"
// "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"

pub use chrono::{NaiveDate, Utc};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        Self {
            form_values: (field_name.to_string(), field_value.to_string()),
            date: now(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Self {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            return Err(FormError {
                form_values: ("first_name".to_owned(), "".to_owned()),
                date: now(),
                err: "No user name".to_owned(),
            });
        }

        // the password must have at least 8 characters, and a combination of alphabetic, numeric and none-alphanumeric (<, &, / ...).

        if self.password.len() < 8 {
            return Err(FormError {
                form_values: ("password".to_owned(), self.password.to_owned()),
                date: now(),
                err: "At least 8 characters".to_owned(),
            });
        }
        let mut have_alphabetic = false;
        let mut have_numeric = false;
        let mut have_none_alphanumeric = false;

        for c in self.password.chars() {
            if have_alphabetic && have_numeric && have_none_alphanumeric {
                break;
            }
            have_alphabetic = have_alphabetic || c.is_alphabetic();
            have_numeric = have_numeric || c.is_numeric();
            have_none_alphanumeric =
                have_none_alphanumeric || !(c.is_alphabetic() || c.is_numeric());
        }
        if have_alphabetic && have_numeric && have_none_alphanumeric {
            Ok(Vec::from(["Valid first name", "Valid password"]))
        } else {
            Err(FormError {
                form_values: ("password".to_owned(),self.password.to_owned()),
                date: now(),
                err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_owned()
            })
        }
    }
}

fn now() -> String {
    Utc::now()
        .naive_local()
        .to_string()
        .rsplit_once('.')
        .unwrap()
        .0
        .to_owned()
}
fn create_date(t: &str) -> NaiveDate {
    NaiveDate::parse_from_str(t, "%Y-%m-%d").unwrap()
}

// fn create_date_time(t: &str) -> NaiveDate {
//     NaiveDate::parse_from_str(t, "%Y-%m-%d %h:%m:%s").unwrap()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut form_output = Form::new(
            String::from("Lee"),
            String::from("Silva"),
            create_date("2015-09-05"),
            String::from("Africa"),
            String::from("qwqwsa1dty_"),
        );

        assert_eq!(
            Form {
                first_name: "Lee".to_string(),
                last_name: "Silva".to_string(),
                birth: create_date("2015-09-05"),
                birth_location: "Africa".to_string(),
                password: "qwqwsa1dty_".to_string()
            },
            form_output
        );
        assert_eq!(
            ["Valid first name".to_string(), "Valid password".to_string()].to_vec(),
            form_output.validate().unwrap()
        );

        form_output.first_name = String::from("");
        assert_eq!(
            FormError {
                form_values: ("first_name".to_owned(), "".to_owned()),
                date: now(),
                err: "No user name".to_owned(),
            },
            form_output.validate().unwrap_err()
        );

        form_output.first_name = String::from("as");
        form_output.password = String::from("dty_1");
        assert_eq!(
            FormError {
                form_values: ("password".to_owned(), "dty_1".to_owned()),
                date: now(),
                err: "At least 8 characters".to_owned(),
            },
            form_output.validate().unwrap_err()
        );

        form_output.password = String::from("asdasASd(_");
        assert_eq!(FormError {
            form_values: ("password".to_owned(), "asdasASd(_".to_owned()),
            date: now(),
            err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_owned() },
                   form_output.validate().unwrap_err()
        );

        form_output.password = String::from("asdasASd123SA");
        assert_eq!(FormError {
                form_values: ("password".to_owned(), "asdasASd123SA".to_string()),
                date: now(),
                err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()
            },
            form_output.validate().unwrap_err()
        );
    }
}

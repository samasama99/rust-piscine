// Create a function named fetch_data which has two arguments:
//
//     server: Which is a Result having the server url or an error message inside.
//     security_level: Which is an enum defining the behavior of the function in case of errors.
//
// The security_level will work as follow:
//
//     Unknown: The function panics without printing any custom message.
//     High: The function panics and prints the error message ERROR: program stops.
//     Medium: The function returns the string WARNING: check the server.
//     Low: The function returns the string Not found: [SERVER_URL].
//     BlockServer: The function will panic only if the Result value is Ok and the error message will be the string contained in Ok.
//
// To return from fetch_data you must use expect, unwrap_or, unwrap_err, unwrap and unwrap_or_else.

pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match (server, security_level) {
        (_, Security::Unknown) => panic!(""),
        (_, Security::High) => panic!("ERROR: program stops"),
        (Ok(url), Security::Medium | Security::Low) => url,
        (Err(_), Security::Medium) => "WARNING: check the server".to_string(),
        (Err(url), Security::Low) => format!("Not found: {url}").to_string(),
        (Ok(url) | Err(url), Security::BlockServer) => panic!("{}", url),
    }
    // match security_level {
    //     Security::Unknown => panic!(""),
    //     Security::High => panic!("ERROR: program stops"),
    //     Security::Medium | Security::Low if server.is_ok() => server.unwrap(),
    //     Security::Medium => server.unwrap_or("WARNING: check the server".to_owned()),
    //     Security::Low => format!("Not found: {}", server.unwrap_err()).to_string(),
    //     Security::BlockServer => panic!("{}", server.unwrap_err()),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "")]
    fn should_panic_test_1() {
        // Panics with no custom message
        fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);
    }

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn should_panic_test_2() {
        // Panics with the message "ERROR: program stops"
        fetch_data(Err(String::new()), Security::High);
    }

    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn should_panic_test_3() {
        // Panics with the message "malicious_server.com"
        fetch_data(
            Ok("malicious_server.com".to_string()),
            Security::BlockServer,
        );
    }

    #[test]
    fn should_work_test_1() {
        assert_eq!(
            "server1.com",
            fetch_data(Ok("server1.com".to_string()), Security::Medium)
        );
    }
    #[test]
    fn should_work_test_2() {
        assert_eq!(
            "WARNING: check the server",
            fetch_data(Err(String::new()), Security::Medium)
        );
    }
    #[test]
    fn should_work_test_3() {
        assert_eq!(
            "Not found: server2.com",
            fetch_data(Err("server2.com".to_string()), Security::Low)
        );
    }
}

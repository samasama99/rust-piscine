use std::io;

const RIDDLE : &str =  "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

const ANSWER: &str = "The letter e";

fn main() {
    let mut number_of_tries = 0;
    loop {
        println!("{}", RIDDLE);
        number_of_tries += 1;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("error while reading the answer");
        if answer.trim().eq(ANSWER) {
            println!("Number of trials: {}", number_of_tries);
            break;
        }
    }
}

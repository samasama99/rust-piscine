// Instructions
//
// Let's play a little.
//
// Create a function named score that given a &str, computes the score for that given string as a u64.
//
// Each letter has a value, you just have to sum the values of the letters in the given string.
//
// You will need these:
// Letter 	Value
// A, E, I, O, U, L, N, R, S, T 	1
// D, G 	2
// B, C, M, P 	3
// F, H, V, W, Y 	4
// K 	5
// J, X 	8
// Q, Z 	10

pub fn score(s: &str) -> u64 {
    s.chars()
        .map(|c| c.to_uppercase().to_string().chars().nth(0).unwrap())
        .map(|c| match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, score("a"));
        assert_eq!(0, score("ã ê Á?"));
        assert_eq!(14, score("ThiS is A Test"));
    }
}

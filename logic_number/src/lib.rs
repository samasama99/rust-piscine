// Instructions
//
// In this exercise, the logic for a sequence of numbers will be tested.
// You will have to create a function which will return true
// if the number is the sum of its own digits,
// where each digit is first raised to the power of the number of digits.
//
// Examples:
//
// 9 returns true, because 9 = 9^1 = 9
// 10 returns false, because 10 != 1^2 + 0^2 = 1
// 153 returns true, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 returns false, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

pub fn number_logic(num: u32) -> bool {
    let n = num;
    let num = num.to_string();
    let power = num.len();
    num.to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap().pow(power as u32))
        .sum::<u32>()
        == n
}

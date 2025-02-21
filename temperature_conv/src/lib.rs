/* formula: (32°F − 32) × 5/9 = 0°C */
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32f64) * 5f64 / 9f64
}

/* formula: (0°C × 9/5) + 32 = 32°F    */
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9f64 / 5f64 + 32f64
}

#[cfg(test)]
mod tests {

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert!(approx_eq(fahrenheit_to_celsius(-459.67), -273.15));
        assert!(approx_eq(fahrenheit_to_celsius(32.0), 0.0));
        assert!(approx_eq(fahrenheit_to_celsius(212.0), 100.0));
        assert!(approx_eq(fahrenheit_to_celsius(98.6), 37.0));
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!(approx_eq(celsius_to_fahrenheit(-273.15), -459.67));
        assert!(approx_eq(celsius_to_fahrenheit(0.0), 32.0));
        assert!(approx_eq(celsius_to_fahrenheit(100.0), 212.0));
        assert!(approx_eq(celsius_to_fahrenheit(37.0), 98.6));
    }
}

// - tests::test_f_to_c stdout ----
// F = -6.666666666666667°C
// ead 'tests::test_f_to_c' panicked at src/main.rs:22:9:
// ertion failed: eql(fahrenheit_to_celsius(temp_f), -6.666666666666666)
// e: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

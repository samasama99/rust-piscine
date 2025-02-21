/* formula: (32°F − 32) × 5/9 = 0°C */
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

/* formula: (0°C × 9/5) + 32 = 32°F    */
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9. / 5. + 32.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(-459.67), -273.15);
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(98.6), 37.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(-273.15), -459.67);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(37.0), 98.6);
    }
}

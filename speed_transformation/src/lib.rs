// km / h = m / s
// 1000 * m / 60 * s = m / s 
pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
   km_h * 1000.0 / 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_per_hour_to_meters_per_second() {
        let km_h = 100.0;
        let expected_m_s = 27.77777777777778;

        let epsilon = 1e-6;

        assert!((km_per_hour_to_meters_per_second(km_h) - expected_m_s).abs() < epsilon);
    }

    #[test]
    fn test_zero_km_per_hour() {
        let km_h = 0.0;
        let expected_m_s = 0.0;
        let epsilon = 1e-6;
        assert!((km_per_hour_to_meters_per_second(km_h) - expected_m_s).abs() < epsilon);
    }

    #[test]
    fn test_one_km_per_hour() {
        let km_h = 1.0;
        let expected_m_s = 0.2777777777777778;
        let epsilon = 1e-6;
        assert!((km_per_hour_to_meters_per_second(km_h) - expected_m_s).abs() < epsilon);
    }
}




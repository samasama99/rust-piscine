pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(234, 2), 236);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_sum_overflow() {
        sum(1, 255);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_diff_overflow() {
        diff(-32768, 32766);
    }

    #[test]
    fn test_product() {
        assert_eq!(pro(23, 2), 46);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_product_overflow() {
        pro(-128, 2);
    }

    #[test]
    fn test_quotient() {
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(quo(-128.23, 2.0), -64.115);
    }

    #[test]
    fn test_remainder() {
        assert_eq!(rem(22.0, 2.0), 0.0);
        assert_eq!(rem(-128.23, 2.0), -0.22999573);
    }
}

/*
 *
 * ate the following functions. The objective is to know how ownership works with different types.
 *
 *     nbr_function returns a tuple:
 *         with the original value.
 *         the exponential function of the value.
 *         and the natural logarithm of the absolute value.
 *     str_function returns a tuple:
 *         with the original value.
 *         and the exponential function of each value as a string (see the example).
 *     vec_function returns a tuple:
 *         with the original value.
 *         and the natural logarithm of each absolute value.
 *
*/

use core::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let second = a
        .split(" ")
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, second)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let second = b.iter().map(|&v| v as f64).map(|v| v.abs().ln()).collect();
    (b, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4, 5];
        let c = 0;

        assert_eq!((0, 1.0, f64::NEG_INFINITY), nbr_function(c));
        assert_eq!(
            (
                [1, 2, 4, 5].to_vec(),
                [
                    0.0,
                    0.6931471805599453,
                    1.3862943611198906,
                    1.6094379124341003
                ]
                .to_vec()
            ),
            vec_function(b)
        );
        let res = str_function(a);
        let tmp = (res.0.as_ref(), res.1.as_ref());
        assert_eq!(("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351"), tmp);
    }
}

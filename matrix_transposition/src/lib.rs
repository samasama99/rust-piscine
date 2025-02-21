#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(transpose(Matrix((1, 3), (4, 5))), Matrix((1, 4), (3, 5)));
    }
}

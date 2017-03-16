/// Struct declaring a matrix as its content
pub struct Matrix<T> {
  pub matrix: Vec<T>,
}

/// constructor
impl<T> Matrix<T> {
  pub fn new(data:Vec<T>) -> Matrix<T> {
    Matrix {
      matrix: data,
    }
  }
}
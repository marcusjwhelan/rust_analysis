use std;

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

impl<T> std::fmt::Display for Matrix<T> where T: std::fmt::Display + std::fmt::Debug{
  fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f,"")?;
    std::fmt::Display::fmt(&self,f);
  }
}
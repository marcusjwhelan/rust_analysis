/// Struct declaring a matrix as its content
pub struct Matrix {
  pub matrix: Vec<Vec<String>>
}

/// constructor
impl Matrix {
  pub fn new(data:Vec<Vec<String>>) -> Matrix {
    Matrix {
      matrix: data
    }
  }
}
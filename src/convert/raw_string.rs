
/// Struct declaring a matrix as its content
pub struct Matrix {
  pub matrix: Vec<Vec<String>>
}

impl Matrix {
  pub fn new(data:Vec<Vec<String>>) -> Matrix {
    Matrix {
      matrix: data
    }
  }
}

/// Convert a csv String into a Matrix;
pub fn to_matrix(raw_string: &String) -> Matrix {
  
  let twod_matrix = raw_string.lines()
    .map(|s| s.trim()
              .split(',')
              .map(String::from)
              .collect::<Vec<String>>())
    .collect::<Vec<Vec<String>>>();
  
  Matrix::new(twod_matrix)
}
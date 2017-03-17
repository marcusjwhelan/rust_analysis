use ::structs::get::Matrix;

/// Convert a csv String into a Matrix;
pub fn to_matrix(raw_string: &String) -> Matrix<Vec<String>> {
  
  let twod_matrix:Vec<Vec<String>> = raw_string.lines()
    .map(|s| s.trim()
              .split(',')
              .map(String::from)
              .collect::<Vec<String>>())
    .collect::<Vec<Vec<String>>>();
  
  let x: Matrix<Vec<String>> = Matrix::new(twod_matrix);
  x
}
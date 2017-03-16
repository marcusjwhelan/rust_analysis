use ::structs::get::Matrix;

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
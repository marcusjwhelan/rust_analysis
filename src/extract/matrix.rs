use ::structs::get::Matrix;

/// Extract a 2d matrix from another 2d 
/// matrix. Omit headers if true;
pub fn twod_data<T> (headers:bool, data:Matrix<T>) -> Matrix<T> {

  for arg in data.matrix.iter().skip(1) {
    println!("{}", arg);
  }

  let l:Matrix<T> = Matrix::new(data.matrix);
  l
}
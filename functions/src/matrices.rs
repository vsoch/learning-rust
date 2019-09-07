// matrices.rs

// A Matrix type is a Vector of vectors of floats
pub type Matrix = Vec<Vec<f32>>;

// Compute the product of two matrices
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
  println!("Matrix 1 {:?}", mat1);  
  println!("Matrix 2 {:?}", mat2);  
  assert_eq!(mat1.len(), mat2.len());

  // Generate empty matrix to fill
  let mut product = mat_mult_base(&mat1, &mat2);
  println!("Empty Matrix to fill {:?}", product);  

  for r in 0..mat1.len() {
    for c in 0..mat2[0].len() {
      for i in 0..mat1.len() {
        product[r][c] += mat1[r][i] * mat2[i][c];       
      }
    }
  }

  println!("Finished matrix: {:?}", product);
  return product;
}

// Generate an empty matrix to fill for matrix multiplication
// mxn X nxp = mxp 
fn mat_mult_base(mat1: &Matrix, mat2: &Matrix) -> Matrix {
  let rows = mat1.len();
  let cols = mat2[0].len();
  let mut product = vec![vec![0.; rows]; cols];
  return product;
}

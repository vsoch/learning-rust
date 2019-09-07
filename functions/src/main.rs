pub mod numbers;
pub mod matrices;

fn main(){

  let array = [1,2,3,4,4,5,1];
  let mat1 = vec![vec![1.;3]; 3];
  let mat2 = vec![vec![5.;3]; 3];
  let vs = array.to_vec();

  // Sum numbers
  numbers::sum(&array);

  // Remove dusplicates, retain order
  numbers::dedup(&vs);

  // Filter out non even numbers
  numbers::filter(&vs, &numbers::is_even);
  matrices::mat_mult(&mat1, &mat2);

}

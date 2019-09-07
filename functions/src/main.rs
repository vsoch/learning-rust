pub mod numbers;

fn main(){

  let array = [1,2,3,4,4,5,1];
  let vs = array.to_vec();

  // Sum numbers
  numbers::sum(&array);

  // Remove dusplicates, retain order
  numbers::dedup(&vs);

  // Filter out non even numbers
  numbers::filter(&vs, &numbers::is_even);
}

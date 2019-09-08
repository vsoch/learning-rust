pub mod numbers;
pub mod matrices;
pub mod games;

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

  // Matrix multiplication
  matrices::mat_mult(&mat1, &mat2);

  // Sieve of Eratosthenes
  games::sieve(7);

  // Towers of Hanoi
  let moves = games::hanoi(3, games::Peg::A, games::Peg::B, games::Peg::C);
  println!("Final moves are {:?}", moves);

}

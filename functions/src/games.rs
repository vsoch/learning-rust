// games.rs

/// Find all prime numbers less than n.
pub fn sieve(n: u32) -> Vec<u32> {
  let mut skip = vec![];
  let mut primes = vec![];

  println!("Starting number for sieve is {}", n);

  // If the number is less than 2, just return
  if n < 2 {
    return primes;
  }

  for x in 2..n {

    // If i has been crossed-out from previous iterations, skip it.
    if skip.contains(&x) {
      continue;
    } 
    primes.push(x);

    // If not crossed-out, then prime..
    for y in x*x..n {

        // Cross-out all multiples of i from i*i to n. These are non-prime
        if y % x == 0 {
          skip.push(y);
        }
    }
  }
  println!("Final list of primes is {:?}", primes);
  return primes;
}


/// This is an enum, reference like `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between a source and destination
pub type Move = (Peg, Peg);

// Given a number of disks, a source and destination, return vector of moves
pub fn hanoi(num_disks: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {

  let mut moves = vec![];

  // If we have one disk, move it and return
  if num_disks == 1 {
    moves.push((src, dst));      
  } else {
    moves.append(&mut hanoi(num_disks - 1, src, dst, aux));   // Step 1
    moves.push((src, dst));                                   // Step 2
    moves.append(&mut hanoi(num_disks - 1, aux, dst, src));   // Step 1
  }
  return moves;
}

// games.rs

//// Find all prime numbers less than n.
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

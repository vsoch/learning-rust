pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Given a number, find the sum of all the unique multiples of particular numbers up to
    // but not including that number.

    (0..limit)
        .filter(|&count| has_divisors(count, factors))
        .sum()
}

pub fn has_divisors(number: u32, factors: &[u32]) -> bool {
    factors
        .iter()
        .any(|&factor| factor > 0 && number % factor == 0)
}

pub fn nth(n: u32) -> u32 {
    // Given a number n, determine what the nth prime is.

    // If the user wants the first prime (index 0) we skip the loop and return this
    let mut next: u32 = 2;
    let mut index = 0;

    // Look through all numbers less than n
    while index < n {
        // If the user wants the second prime (n=1) increment next so we don't return 2
        next += 1;
        while !is_prime(next) {
            next += 1;
        }
        index += 1;
    }
    next
}

pub fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

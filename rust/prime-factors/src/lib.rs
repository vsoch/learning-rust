pub fn factors(n: u64) -> Vec<u64> {
    let mut current = n;

    // start testing with 2
    let mut factor = 2;

    // keep a vector list of results
    let mut results = vec![];

    // keep going until we hit 1
    while current > 1 {
        // if divides cleanly, divide it
        if current % factor == 0 {
            current /= factor;
            results.push(factor);
        } else {
            factor += 1;
        }
    }
    results
}

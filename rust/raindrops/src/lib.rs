pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => result.push_str("PlingPlangPlong"),
        (0, 0, _) => result.push_str("PlingPlang"),
        (0, _, 0) => result.push_str("PlingPlong"),
        (_, 0, 0) => result.push_str("PlangPlong"),
        (0, _, _) => result.push_str("Pling"),
        (_, 0, _) => result.push_str("Plang"),
        (_, _, 0) => result.push_str("Plong"),
        (_, _, _) => result = n.to_string(),
    }
    result
}

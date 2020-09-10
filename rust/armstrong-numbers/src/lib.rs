pub fn is_armstrong_number(num: u32) -> bool {
    // An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.
    let mut characters = vec![];
    let mut powers = vec![];
    const RADIX: u32 = 10;

    // Add characters to characters vector
    for c in num.to_string().chars() {
        characters.push(c);
    }
    for s in characters.iter() {
        powers.push(s.to_digit(RADIX).unwrap().pow(characters.len() as u32));
    }
    let sum: u32 = powers.iter().sum();
    sum == num
}

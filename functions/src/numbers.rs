// numbers.rs

// How to import: use:numbers::sum;
//                use:numbers::{sum, multiply};
//                use:numbers::*
//                use:numbers  (then reference numbers::sum()

// Take a list (slice) of integers and output an integer (sum)
pub fn sum(slice: &[i32]) -> i32 {
    let sum: i32 = slice.iter().sum();
    println!("The sum is {}", sum);
    return sum;
}

pub fn is_even(num: i32) -> bool {
    return (num % 2) == 0;
}

// Deduplicate items in the input vector 'vs'.
// Produces a vector with the first instance of each unique, in original order
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    println!("Starting vector is {:?}", vs);
    let mut v: Vec<i32> = vs.to_vec();
    v.sort();
    v.dedup();
    println!("Deduped vector is {:?}", v);
    return v;
}

// Filters a vector using a function (a predicate) return
// a new vector with elements that satisfy pred
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    println!("Before filtering {:?}", vs);
    let filtered: Vec<i32> = vs.to_vec().into_iter().filter(|voc| pred(*voc)).collect();
    println!("After filtering {:?}", filtered);
    return filtered;
}

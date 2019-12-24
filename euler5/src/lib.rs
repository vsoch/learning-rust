
// Library helping functions

pub fn test_number(number: i32) -> bool {
    for n in 1..=20 {
        if number % n != 0 {
            return false;
        }    
    }
    true
}

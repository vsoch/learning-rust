// 2520 is the smallest number that can be divided by each of the 
// numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the 
// numbers from 1 to 20?

use euler5::test_number;

fn main() {

    // Start at 2520 since we know divisible by 1 through 10
    let mut number = 2520;

    loop {
        if test_number(number) {
            println!("Number is {}", number);
            break;
        }
        number+=1;
    }
}

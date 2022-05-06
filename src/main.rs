use std::io;
use prime_finder::finds_if_number_is_prime;
fn main() {
    let user_given_number = asks_user_for_number();
    let divisor = finds_if_number_is_prime(user_given_number);
        
    if divisor != 1 {
    if divisor != 0 {
            println!("{} is not prime, it is divisible by {}", user_given_number, divisor)
    }
    else {
            println!("1 is not prime, because it's the zero (which is niether positive, nor negative) of multiplication, and therefore is niether prime nor composite. (niether is a building block, nor is built)")
    }
    }
    else {
        println!("{} is prime.", user_given_number);
    }
}


fn asks_user_for_number() -> u128 {
    loop {
        println!("Enter any positive integer less than 2^128");
        let mut string_to_recieve_input = String::new();
            io::stdin()
                .read_line(&mut string_to_recieve_input)
                .expect("Failed to read input");
            let user_input:u128 = match string_to_recieve_input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        if user_input > 0 {
            return user_input
        }
        else {
            println!("Not a positive integer less than 2^128, try again.")
        }
    }
}


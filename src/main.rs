use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;
use prime_finder::finds_if_number_is_prime;
use prime_finder::collect_primes;




fn main() {
    let modes = vec!["check".to_string(),"collect".to_string()];
    let mode = asks_user_for_mode(modes);
    if mode == "check" {
        check_a_number()
    }
    else {
        if mode == "collect" {
            collect_numbers()
        }
        else {
            panic!("not a mode, something has gone wrong!")
        }
    }
}




fn check_a_number() {
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




fn collect_numbers() {
    let mut file = fs::File::open("primes_list")
    .expect("File not found");
    let mut primes_list = vec![];
    let mut primes_list_string = String::new();
    file.read_to_string(&mut primes_list_string);
    for number in primes_list_string.lines() {
        primes_list.push(number.parse::<u128>().unwrap()) 
    }
    //let primes_list = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359];
    //let mut primes_list_new = vec![];
    let mut primes_list_new = collect_primes(&primes_list);
    //println!("{:?}",primes_list)
    let mut primes_list_string_out = String::new();
    primes_list_string_out.push_str(format!("{}",primes_list[0]).as_str());
    primes_list.remove(0);
    for prime in primes_list {
        primes_list_string_out.push_str(format!("\n{}",prime).as_str())
    }
    for prime in primes_list_new {
        primes_list_string_out.push_str(format!("\n{}",prime).as_str());
    }
    println!("{}",primes_list_string_out);
    write!(fs::OpenOptions::new()
      .write(true)
      //.append(true)
      .open("primes_list")
      .unwrap(), "{}",primes_list_string_out).unwrap()
}




fn asks_user_for_number() -> u128 {
    loop {
        println!("Enter any natural number less than 2^128");
        let mut string_to_recieve_input = String::new();
            io::stdin()
                .read_line(&mut string_to_recieve_input)
                .expect("Failed to read input");
            let user_input:u128 = match string_to_recieve_input.trim().parse::<u128>() {
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




fn asks_user_for_mode(list_of_modes:Vec<String>) -> String {
    loop {
        println!("Enter a mode from the following list");
        for mode in &list_of_modes {
        println!("{}",mode);
        }
        let mut string_to_recieve_input = String::new();
            io::stdin()
                .read_line(&mut string_to_recieve_input)
                .expect("Failed to read input");
            let user_input:String = string_to_recieve_input.trim().parse::<String>().expect("an internal error occured");
        for mode in &list_of_modes {
            if user_input == *mode {
                return user_input
            }
        }
        println!("Not an available mode, try again.")
    }
}

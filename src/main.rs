use std::io;
use std::thread;
use num_integer::Roots;
use std::sync::mpsc;
use num_cpus::get;
use indicatif::ProgressBar;

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


fn finds_if_number_is_prime(number_to_check:u128) -> u128 {
    if number_to_check == 1 {
        return 0;
    }
    else {
        let number_of_threads:u128 = ((get()) as u128) * 2;
        println!("spinning up {} threads", number_of_threads);
        let bar = ProgressBar::new(number_to_check.sqrt().try_into().unwrap());
        let (tx, rx) = mpsc::channel();
        let mut count2:u128 = 1;
        //let (graphtx, graphrx) = mpsc::channel();
        let mut threads_group = vec![];
        for thread_number in 0..number_of_threads {
            let tx1 = tx.clone();
            let bar1 = bar.clone();
            // Spin up another thread
            threads_group.push(thread::spawn(move || {
                 let root:u128 = number_to_check.sqrt().try_into().unwrap();
                 let mut count = 3 + (thread_number * 2);
                 
                 if 2 > root {
                    match tx1.send(1) {
                        Ok(()) => 2,
                        Err(_) => 0,
                    };
                    return (true, 1);
                 }
                 if number_to_check % 2 == 0 {
                     match tx1.send(2) {
                        Ok(()) => 2,
                        Err(_) => 0,
                     };
                     return (false, 2);
                 }

                 loop {

                    if count > root {
                        match tx1.send(1) {
                            Ok(()) => 2,
                            Err(_) => 0,
                        };
                     return (true, 1);
                     }       
                 
                     if number_to_check % count == 0 {
                        match tx1.send(count) {
                            Ok(()) => 2,
                            Err(_) => 0,
                        };
                        return (false, count);
                     }

                     if count2 < 2815369 {
                     }
                     else {
                         bar1.inc(5630738);
                        count2 = 1;
                    }

                     count = count + (number_of_threads * 2);
                     count2 = count2 + 1;
                   //  graphtx1.send(count);
                 //  bar1.inc(1);
                 }

            }));
        }
        let mut andy:bool = true;
        let mut real_divisor:u128 = 1;
        println!("threads started");
        let received:u128 = match rx.recv() {
            Ok(num) => num,
            Err(_) => 0,
        };
        //println!("recieved message OR burrito packaged");
        if received == 1 {
           // println!("recieved 1");
            for this_thread in threads_group {
                // Wait for the thread to finish. Returns a result.
                let this_thread_output = match this_thread.join() {
                    Ok((bool, u128)) => ((bool, u128)),
                    Err(_) => (false, 1),
                };
                let (primeliness, divisor) = this_thread_output;

                andy = andy && primeliness;
                if real_divisor == 1 {
                    if andy != true {
                        real_divisor = divisor;
                      //  return (andy, realdivis);
                    }
                }
            }
        }
        else {
            if received != 0 {
            for thread_number in 0 .. number_of_threads {
            println!("letting thread {} know",thread_number)
    
            }

            real_divisor = received;
            //andy = false;
            }
            else {
            println!("there was an issue recieving the divisor from a thread");
            }
        }
real_divisor
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let divisor = finds_if_number_is_prime(4);
        assert_eq!(2, divisor);
    }
}


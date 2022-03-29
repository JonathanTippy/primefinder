use std::io;
use std::thread;
use num_integer::Roots;
//use std::time;
use std::sync::mpsc;
use num_cpus::get;
use indicatif::ProgressBar;


fn main() {
    let input = askuser();
    let (prime, divis) = numcheck(input);
    if prime == false {
        if divis != 18446744073709551616 {
            println!("{} is not prime, it is divisible by {}", input, divis)
        }
        else {
            println!("1 is not prime, because it's the zero (which is niether positive, nor negative) of multiplication, and therefore is niether prime nor composite. (niether is a building block, nor is built)")
        }
    }
    else {
        println!("{} is prime.", input)
    }
}

fn numcheck(input:u128) -> (bool, u128) {
    if input == 1 {
        return (false, 18446744073709551616);
    }
    else {
        let n_threads:u128 = ((get()) as u128) * 2;
        println!("spinning up {} threads", n_threads);
        let bar = ProgressBar::new(input.sqrt().try_into().unwrap());
        let (tx, rx) = mpsc::channel();
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];
        for i in 0..n_threads {
        let tx1 = tx.clone();
        let bar1 = bar.clone();
            // Spin up another thread
            children.push(thread::spawn(move || {
                 let root:u128 = input.sqrt();
                 let mut count = 3 + (i * 2);
                let mut count2 = 1;
                 
                 if count > root {
                    match tx1.send(1) {
                        Ok(()) => 2,
                        Err(_) => 0,
                    };
                    bar1.finish();
                    return (true, count);
                 }

                 if input % 2 == 0 {
                     match tx1.send(count) {
                        Ok(()) => 2,
                        Err(_) => 0,
                     };
                     bar1.finish();
                     return (false, count);
                 }

                 loop {

                    if count > root {
                        match tx1.send(1) {
                            Ok(()) => 2,
                            Err(_) => 0,
                        };
                        bar1.finish();
                     return (true, count);
                     }       
                 
                     if input % count == 0 {
                        match tx1.send(count) {
                            Ok(()) => 2,
                            Err(_) => 0,
                        };
                        bar1.finish();
                        return (false, count);
                     }

                     if count2 != 10000000 {
                     }
                     else {
                       // if i == 0 {
                         bar1.inc(20000000);
                       // }
                        count2 = 1;
                     }

                     count = count + (n_threads * 2);
                     count2 = count2 + 1;
                 }

            }));
        }
        let mut andy:bool = true;
        let mut realdivis:u128 = 1;
        let received = match rx.recv() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if received == 1 {
            for child in children {
                // Wait for the thread to finish. Returns a result.
                let result = match child.join() {
                    Ok((bool, u128)) => ((bool, u128)),
                    Err(_) => (false, 1),
                };
                let (prime, divis) = result;

                andy = andy && prime;
                if realdivis == 1 {
                    if andy != true {
                        realdivis = divis;
                        return (andy, realdivis);
                    }
                }
            }
        }
        else {
            if received != 0 {
            realdivis = received;
            andy = false
            }
            else {
            println!("there was an issue recieving the divisor from a thread");
            }
        }
(andy, realdivis)
    }
}

fn askuser() -> u128 {
    loop {
        println!("Enter any positive integer less than 2^128");
        let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read input");
            let number:u128 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        if number > 0 {
            return number
        }
        else {
            println!("Not a positive integer less than 2^128, try again.")
        }
    }
}


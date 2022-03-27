use std::io;
use std::thread;
use num_integer::Roots;
//use std::time;
use std::sync::mpsc;
//use num_cpus::get;

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
        let n_threads:u128 = ((num_cpus::get()) as u128) * 2;
        println!("spinning up {} threads", n_threads);
        let (tx, rx) = mpsc::channel();
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];
        for i in 0..n_threads {
        let tx1 = tx.clone();
            // Spin up another thread
            children.push(thread::spawn(move || {
                 
                 let root:u128 = input.sqrt();
                 let mut count = 3 + (i * 2);
                
                 if input % 2 != 0 {
                 }
                 else {
                     if input != 2 {
                    tx1.send(2).unwrap();
                    return (false, 2)
                     }
                     else {
                     }
                 }

                 loop {
                     if count <= root {
                     }
                     else {
                         tx1.send(1).unwrap();
                         return (true, count);
                     }
                     //println!(" checking {}", count);
                     if input % count != 0 {
                      }
                      else {
                          tx1.send(count).unwrap();
                          return (false, count);
                     }
                     count = count + (n_threads * 2);
                 }

            }));
        }
        let mut andy:bool = true;
        let mut realdivis:u128 = 1;
        let received = rx.recv().unwrap();
        if received == 1 {
            for child in children {
                // Wait for the thread to finish. Returns a result.
                let result = child.join().unwrap();
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
            realdivis = received;
            andy = false
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


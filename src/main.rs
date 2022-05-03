use std::io;
use std::thread;
use num_integer::Roots;
use num_cpus::get;
use indicatif::ProgressBar;
use ringbuf::RingBuffer;
use std::time::Duration;
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
        let progress_bar = ProgressBar::new(number_to_check.sqrt().try_into().unwrap());
        let mut count2:u128 = 1;
        let mut thread_group_ring_buffers_divisor = vec![];
        let mut thread_group_ring_buffers_stop = vec![];
        let mut threads_group = vec![];
        for thread_number in 0..number_of_threads {
            let progress_bar_clone = progress_bar.clone();
            let this_thread_ring_buffer_divisor = RingBuffer::<u128>::new(1);
            let this_thread_ring_buffer_stop = RingBuffer::<bool>::new(1);
            let (mut this_thread_ring_buffer_divisor_write, this_thread_ring_buffer_divisor_read) = this_thread_ring_buffer_divisor.split();
            let (this_thread_ring_buffer_stop_write, this_thread_ring_buffer_stop_read) = this_thread_ring_buffer_stop.split();
            thread_group_ring_buffers_divisor.push(this_thread_ring_buffer_divisor_read);
            thread_group_ring_buffers_stop.push(this_thread_ring_buffer_stop_write);
            threads_group.push(thread::spawn(move || {
                 let root:u128 = number_to_check.sqrt().try_into().unwrap();
                 let mut count = 3 + (thread_number * 2); 
                 if 2 > root {
                     this_thread_ring_buffer_divisor_write.push(1).unwrap();
                     return (true, 1);
                 }
                 if number_to_check % 2 == 0 {
                     this_thread_ring_buffer_divisor_write.push(2).unwrap();
                     return (false, 2);
                 }

                 loop {

                    if count > root {
                        this_thread_ring_buffer_divisor_write.push(1).unwrap();
                        return (true, 1);
                     }       
                 
                     if number_to_check % count == 0 {
                        this_thread_ring_buffer_divisor_write.push(count).unwrap();
                        return (false, count);
                     }

                     if count2 < 2815369 {
                     }
                     else {
                        if this_thread_ring_buffer_stop_read.is_empty() {
                        }
                        else {
                        println!("recieved stop command, thread {} stopping",thread_number);
                        panic!()
                        }
                        progress_bar_clone.inc(5630738);
                        count2 = 1;
                    }

                     count = count + (number_of_threads * 2);
                     count2 = count2 + 1;
                 }

            }));
        }
        let mut andy:bool = true;
        let mut real_divisor:u128 = 1;
        let mut received = 0;
        println!("threads started");
        loop {
        for this_thread_ring_buffer_divisor_read in &mut thread_group_ring_buffers_divisor {
       // let (mut this_thread_ring_buffer_write, mut this_thread_ring_buffer_read) = this_thread_ring_buffer.split();
            if this_thread_ring_buffer_divisor_read.is_empty() {
                thread::sleep(Duration::from_millis(1));
            }
            else {
                received = this_thread_ring_buffer_divisor_read.pop().unwrap();
            }
        }
        if received != 0 {
        break
        }
        }
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
            for this_thread_ring_buffer_stop_write in &mut thread_group_ring_buffers_stop {
                this_thread_ring_buffer_stop_write.push(true).unwrap();
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


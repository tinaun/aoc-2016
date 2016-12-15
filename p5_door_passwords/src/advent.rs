/// advent of code 
///
/// problem 5, part 1 (for now)
///
/// generate password from consecutive md5 hashes,
/// salted with input - the door id

pub static PROBLEM_NUMBER: &'static str = "5"; 
static NUM_CPUS: usize = 2;

use md5;

use std::thread;
use std::io;
use std::io::Write;
use std::sync::{Arc, Barrier, mpsc};

use std::collections::BTreeMap;

pub fn adv_main(input: Vec<String>) {
    println!("testing {}", input[0]);
    let initial = input[0].clone().into_bytes();
    let mut password = BTreeMap::new();

    let (tx, rx) = mpsc::channel();
    let barrier = Arc::new(Barrier::new(NUM_CPUS));

    //println!("test: {}, {:?}", "abc3231929", md5::compute(b"abc3231929"));

    for i in 0..NUM_CPUS {
        let sender = tx.clone();
        let initial = initial.clone();
        let c = barrier.clone();

        thread::spawn(move || {
            let mut counter = i;
            loop {
                let string = [initial.clone(), format!("{}", counter).into_bytes()].concat();
                let hash = md5::compute(&string[..]);

                if &hash[..2] == &[0,0] && hash[2] < 16 {
                    sender.send( (counter, hash[2]) ).unwrap();

                }

                //c.wait();

                counter += NUM_CPUS;
            }
        });
    }

    while let Ok((loc, ch)) = rx.recv() {
        password.insert(loc, format!("{:x}", ch));
        
        //println!("@{} found digit! - {:x}", loc, ch);
        print!("first: {:8}\r", password.values().take(8).cloned().collect::<Vec<_>>().concat());
        let _ = io::stdout().flush().unwrap();

        if password.len() > 8 {
            println!();
            break;
        }
    }
    
    //println!("final password: {:?}", password);
}

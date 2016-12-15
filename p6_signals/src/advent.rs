/// advent of code 
///
/// problem 6
///
/// decode repetition codes

pub static PROBLEM_NUMBER: &'static str = "6"; 

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

pub fn adv_main(input: Vec<String>) {
    
    let length = input[0].len();
    let mut output = vec!['\0'; length];
    let mut least = vec!['\0'; length];

    let data = Arc::new(input);
    let (send, recv) = mpsc::channel();
    

    for i in 0..length {
        let (data, send) = (data.clone(), send.clone());

        thread::spawn(move || {
            let mut count: BTreeMap<char, usize> = BTreeMap::new();

            for j in 0..data.len() {
                *count.entry( data[j].chars().nth(i).unwrap() ).or_insert(0) += 1;
            }

            let mut count = count.into_iter().collect::<Vec<_>>();
            count.sort_by_key(|&(_, n)| 10000 - n);

            let (most_common, _) = count[0];
            let (least_common, _) = count[count.len() - 1];

            send.send( (i, most_common, least_common) ).unwrap();
        });
    }

    for _ in 0..length {
        let (index, chr, lchr) = recv.recv().unwrap();
        output[index] = chr;
        least[index] = lchr;

    }

    println!("decoded: {} {}", output.into_iter().collect::<String>(),
                               least.into_iter().collect::<String>());

}


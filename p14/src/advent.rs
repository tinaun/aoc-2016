/// advent of code 2016
///
/// problem 14
///
/// md5 hash keygen

pub static PROBLEM_NUMBER: &'static str = "14";

use std::collections::VecDeque;

use md5::{self, Digest};

fn generate_hash(salt: &[u8], input: usize) -> Vec<u8> {

    let hash = md5::compute([salt.clone(), &format!("{}", input).into_bytes()].concat());
    let mut hash = format!("{:x}", hash).into_bytes();

    for _ in 0..2016 {
        hash = format!("{:x}", md5::compute(hash)).into_bytes();
    }

    hash
}


fn has_run(n: usize, key: &[u8]) -> Option<u8> {

    for run in key.windows(n) {
        let ch = run[0];

        if run.iter().all(|&c| c == ch) {
            return Some(ch);
        }
    }

    None
}

pub fn adv_main(input: Vec<String>) {
    let salt = input[0].clone().into_bytes();

    let mut search_space: VecDeque<_> = (0..1000).map(|c| generate_hash(&salt, c)).collect();

    let mut index = 0;
    let mut valid_keys = 0;

    while valid_keys < 64 {
        let key = search_space.pop_front().unwrap();
        search_space.push_back(generate_hash(&salt, index + 1000));

        if let Some(ch) = has_run(3, &key) {
            for k in &search_space {
                if let Some(ch2) = has_run(5, k) {
                    if ch == ch2 {
                        valid_keys += 1;
                        println!("the {}th valid key is {}.", valid_keys, index);
                        break;
                    }
                }
            }
        }

        index += 1;
    }
}

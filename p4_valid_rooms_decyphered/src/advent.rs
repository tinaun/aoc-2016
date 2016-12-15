/// advent of code 
///
/// problem 4
///
/// filter and decypher a list of encrypted room names

pub static PROBLEM_NUMBER: &'static str = "4a"; 

use std::collections::BTreeMap;

fn rotate(input: &str, shift: u8) -> String {
    String::from_utf8(
        input.as_bytes().iter().map(|byte| {
            ((byte - 97 + shift) % 26) + 97
        }).collect::<Vec<u8>>()
    ).unwrap_or("whoops, weird string data?".to_string())
}


pub fn adv_main(input: Vec<String>) {
    let mut total: u64 = 0;

    for id in input {
        let mut count: BTreeMap<char, u32> = BTreeMap::new();

        let mut parts: Vec<_> = id.split("-").collect();
        let (rawid, chk) = parts.pop().unwrap().split_at(3);

        for c in parts.concat().chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        //println!("{:?} ({},{})", count, rawid, chk);

        let mut v: Vec<(char, u32)> = count.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));

        let s: String = v.into_iter().take(5).map(|(a, _)| a).collect();
        
        
        //println!("chk: {}, computed: {}", &chk[1..6], s);
        if s == &chk[1..6] {
            let room_number: u64 = rawid.parse().unwrap();
            let shift = room_number % 26;

            let desc = parts.iter().map(|word| rotate(word, shift as u8))
                                   .collect::<Vec<_>>().join(" ");

            if desc.contains("pole") {
                println!("problem b:");
                println!("valid room no. {} - {}", room_number, desc);
            }
            total += room_number;
        }

    }

    println!("total (problem a): {}", total);
}

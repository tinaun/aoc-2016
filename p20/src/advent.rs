/// advent of code 2016 
///
/// problem 20
/// ip range filtering


/// part 1
/// cargo run --release input.txt > output.txt
/// head -1 output.txt

/// part 2
/// wc -l output.txt

pub static PROBLEM_NUMBER: &'static str = "20"; 

use std::collections::BTreeSet;
use std::cmp::max;

pub fn adv_main(input: Vec<String>) {
    let mut ranges: BTreeSet<(u32, u32)> = BTreeSet::new();

    for line in input {
        if line.len() > 1 {
            let limits: Vec<u32> = line.split("-").filter_map(|n| n.parse().ok() ).collect();

            ranges.insert( (limits[0], limits[1]) );
        }
    }

    let mut lasthigh = 0;
    for (l, h) in ranges {
        if l > 0 && (lasthigh) < l-1 {
            println!("gap! - {} ", lasthigh + 1);
        }

        lasthigh = max(h, lasthigh);
    }

}



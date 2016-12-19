use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

extern crate arrayvec;
extern crate itertools;

mod advent;

fn open_file(filename: &str) -> io::Result<Vec<String>> {
    let f = try!(File::open(filename));
    let f = BufReader::new(f);

    let mut lines = Vec::new();

    for line in f.lines() {
        let line = try!(line);
        lines.push(line);
    }

    Ok(lines)
}

fn main() {
    use std::env;
    
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("usage: {} [file] - performs advent of code 2016 problem #{} on input file.", 
                  args[0], advent::PROBLEM_NUMBER);
        return;
    }

    let lines = open_file(&args[1]).unwrap();
    
    advent::adv_main(lines);
}

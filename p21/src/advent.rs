/// advent of code 2016 
///
/// problem 21
/// 

pub static PROBLEM_NUMBER: &'static str = "21"; 

use super::errors::*;

pub fn adv_main(input: Vec<String>) -> Result<(), Error> {
    let mut start = "abcdefgh".to_string();

    for line in &input {
        let words: Vec<&str> = line.split(' ').collect();
        //println!("{}: {:?}", start, words);
        start = match (words[0], words[1]) {
            ("swap", "position") => {
                let (x, y): (usize, usize) = (words[2].parse()?, words[5].parse()?);
                let mut scratch = start.clone().into_bytes();

                let tmp = scratch[x];
                scratch[x] = scratch[y];
                scratch[y] = tmp;

                String::from_utf8(scratch)?
            },
            ("swap", "letter") => {
                let mut scratch = start.replace(words[2], "_");
                scratch = scratch.replace(words[5], words[2]);
                scratch.replace("_", words[5])
            },
            ("rotate", "left") => {
                let n: usize = words[2].parse()?;

                for _ in 0..n {
                    let first = start.remove(0);
                    start.push(first);
                }
                start
            },
            ("rotate", "right") => {
                let n: usize = words[2].parse()?;

                for _ in 0..n {
                    let last = start.pop().ok_or("WTF")?;
                    start.insert(0, last);
                }
                start
            },
            ("rotate", "based") => {
                let n = start.find(words[6]).ok_or("WTF")?;

                let m = if n < 4 {
                    n + 1
                } else {
                    n + 2
                };

                for _ in 0..m {
                    let last = start.pop().ok_or("WTF")?;
                    start.insert(0, last);
                }
                start
            },
            ("reverse", "positions") => {
                 let (x, y): (usize, usize) = (words[2].parse()?, words[4].parse()?);
                 let substr = &start[x..y+1];
                 let rev: String = substr.chars().rev().collect();
                 
                 let end = if y+1 == start.len() {
                     ""
                 } else {
                     &start[y+1..]
                 };

                 [&start[..x], &rev, end].concat()
            },
            ("move", "position") => {
                 let (x, y): (usize, usize) = (words[2].parse()?, words[5].parse()?);

                 let chr = start.remove(x);
                 start.insert(y, chr);
    
                 start
            },
            _ => { start },
        };
    }    

    println!("final: {}", start);


    start = "fbgdceah".to_string();
    //start = "aefgbcdh".to_string();

    for line in input.iter().rev() {
        let words: Vec<&str> = line.split(' ').collect();
        //println!("{}: {:?}", start, words);
        start = match (words[0], words[1]) {
            ("swap", "position") => {
                let (x, y): (usize, usize) = (words[2].parse()?, words[5].parse()?);
                let mut scratch = start.clone().into_bytes();

                let tmp = scratch[x];
                scratch[x] = scratch[y];
                scratch[y] = tmp;

                String::from_utf8(scratch)?
            },
            ("swap", "letter") => {
                let mut scratch = start.replace(words[2], "_");
                scratch = scratch.replace(words[5], words[2]);
                scratch.replace("_", words[5])
            },
            ("rotate", "right") => {
                let n: usize = words[2].parse()?;

                for _ in 0..n {
                    let first = start.remove(0);
                    start.push(first);
                }
                start
            },
            ("rotate", "left") => {
                let n: usize = words[2].parse()?;

                for _ in 0..n {
                    let last = start.pop().ok_or("WTF")?;
                    start.insert(0, last);
                }
                start
            },
            ("rotate", "based") => {
                let n = start.find(words[6]).ok_or("WTF")?;
                let table = [1,1,6,2,7,3,8,4];

                let m = table[n];

                for _ in 0..m {
                    let first = start.remove(0);
                    start.push(first);
                }
                start
            },
            ("reverse", "positions") => {
                 let (x, y): (usize, usize) = (words[2].parse()?, words[4].parse()?);
                 let substr = &start[x..y+1];
                 let rev: String = substr.chars().rev().collect();
                 
                 let end = if y+1 == start.len() {
                     ""
                 } else {
                     &start[y+1..]
                 };

                 [&start[..x], &rev, end].concat()
            },
            ("move", "position") => {
                 let (y, x): (usize, usize) = (words[2].parse()?, words[5].parse()?);

                 let chr = start.remove(x);
                 start.insert(y, chr);
    
                 start
            },
            _ => { start },
        };
    }    

    println!("original: {}", start);

    Ok(())
}



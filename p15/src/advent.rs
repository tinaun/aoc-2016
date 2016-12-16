/// advent of code 2016 
///
/// problem 15
///

pub static PROBLEM_NUMBER: &'static str = "15"; 

pub fn adv_main(input: Vec<String>) {
    let mut discs: Vec<(usize, usize)> = Vec::new();

    for line in input {
        let line = &line[.. line.len()-1];

        let words: Vec<_> = line.split(" ").collect();
        let disc = (words[3].parse().unwrap(), words[11].parse().unwrap());

        discs.push(disc);
    }
    discs.push((11, 0));

    let mut time = 0;
    println!("{:#?}", discs);

    loop {
        if discs.iter().enumerate().all(|(i, &(size, offset))| {
            (offset + i + 1 + time) % size == 0
        }) { break; }

        time += 1;
    }

    println!("first time: {}", time);
}


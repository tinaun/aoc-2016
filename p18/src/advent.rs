/// advent of code 2016 
///
/// problem 18
/// scan room for traps - 2d cellular automata

use itertools::Itertools;

pub static PROBLEM_NUMBER: &'static str = "18"; 

pub fn adv_main(input: Vec<String>) {
    let mut output = vec![input[0].clone()];

    while output.len() < 400000 {
        let last = format!(".{}.", output[output.len() - 1]);
        let mut next = String::new();

        for (a, b, c) in last.chars().tuple_windows() {
            let ch = match (a, b, c) {
                ('^', _, '.') | ('.', _, '^') => '^',
                _                             => '.',
            };

            next.push(ch);
        }
        output.push(next);
    }

    //println!("{:#?}", output);
    println!("count: {}", output.iter().map(|s| { 
        s.chars().filter(|&c| c == '.').count()
    }).sum::<usize>());
}


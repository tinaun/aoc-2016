/// advent of code 
///
/// problem 9
///
/// decompress a compressed file format
/// 

pub static PROBLEM_NUMBER: &'static str = "9"; 

fn parse_marker<T: Iterator<Item=char>>(iter: &mut T) -> (usize, usize) {

    let mut seq = String::new();

    while let Some(next) = iter.next() {
        match next {
            ')' => { break; },
            _ => seq.push(next),
        }
    }
    if seq.len() == 0 {
        return (1, 1);
    }

    let nums: Vec<_> = seq.split("x").collect();

    (nums[0].parse().unwrap(), nums[1].parse().unwrap())
}

fn decompress(input: &str) -> String {
    let mut iter = input.chars();
    let mut output = String::new();

    while let Some(next) = iter.next() {
        
        match next {
            '(' => {
                let (len, rep) = parse_marker(&mut iter);
                let slice = &iter.as_str()[..len];

                for _ in 0..rep {
                    output.push_str(slice);
                }

                let _ = iter.nth(len - 1);
            },
            _ => output.push(next),
        }
    }

    output
}

fn recursive_decompress(input: &str) -> usize {
    //println!("{}", input);

    let mut count = 0;

    let mut iter = input.chars(); 
    while let Some(next) = iter.next() {
        count += match next {
            '(' => {
                let (len, rep) = parse_marker(&mut iter);

                let output_count = recursive_decompress( &iter.as_str()[..len]);
                let _ = iter.nth(len - 1);
                
                output_count * rep
            },
            _ => 1,
        };
        
    }

    count
}

pub fn adv_main(input: Vec<String>) {

    let input = input.concat();                  
    let output = decompress(&input);
    let repeated = recursive_decompress(&input);

    println!("1 iteration len: {}, final len: {}", output.len(), repeated);

}


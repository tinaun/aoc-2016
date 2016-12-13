/// advent of code 2016 
///
/// problem 12
///
/// simple assembly interpreter

pub static PROBLEM_NUMBER: &'static str = "12"; 


pub fn adv_main(input: Vec<String>) {

    let mut input: Vec<Vec<_>> = input.iter().map(|s| s.split(" ").collect()).collect();
    let mut cpu = [0i32, 0i32, 1i32, 0i32]; //part 2

    let parse_reg = |v| -> usize {
        match v {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            _ => panic!(),
        }
    };

    let parse_value = |c: [i32; 4], v| -> i32 {
        match v {
            "a" => c[0],
            "b" => c[1],
            "c" => c[2],
            "d" => c[3],
            _ => v.parse().unwrap(),
        }
    };

    let mut pc: i32 = 0;
    let end = input.len() as i32;

    while pc < end {
        let pcu = pc as usize; 

        let ref words = input[pcu];   

        match words[0] {
            "add" => cpu[ parse_reg(words[2]) ] += parse_value(cpu, words[1]),
            "cpy" => cpu[ parse_reg(words[2]) ] = parse_value(cpu, words[1]),
            "inc" => cpu[ parse_reg(words[1]) ] += 1,
            "dec" => cpu[ parse_reg(words[1]) ] -= 1,
            "jnz" => {
                if parse_value(cpu, words[1]) != 0 {
                    pc += parse_value(cpu, words[2]);
                    continue;
                }
            }
            _ => {},
        }

        pc += 1;
    }

    println!("a: {}", cpu[0]);


}


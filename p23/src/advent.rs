/// advent of code 2016 
///
/// problem 23
///
/// slightly more complicated assembly interpreter

pub static PROBLEM_NUMBER: &'static str = "23"; 

use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reg {
    A, B, C, D,
}

impl From<Reg> for usize {
    fn from(t: Reg) -> Self {
        match t {
            Reg::A => 0,
            Reg::B => 1,
            Reg::C => 2,
            Reg::D => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arg {
    Reg(Reg),
    Value(i32),
}

impl Arg {
    fn get_value(&self, cpu: [i32; 4]) -> i32 {
        match *self {
            Arg::Reg(r) => cpu[ usize::from(r) ],
            Arg::Value(v) => v,
        }
    }
}

impl FromStr for Arg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {        
        match s {
            "a" => Ok(Arg::Reg(Reg::A)),
            "b" => Ok(Arg::Reg(Reg::B)),
            "c" => Ok(Arg::Reg(Reg::C)),
            "d" => Ok(Arg::Reg(Reg::D)),
            s   => {
                match s.parse::<i32>() {
                    Ok(v) => Ok(Arg::Value(v)),
                    Err(_) => Err("invalid register".to_string()),
                }
            }             
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instr {
    Copy(Arg, Arg),
    Increment(Arg),
    Decrement(Arg),
    JmpNotZro(Arg, Arg),
    Toggle(Arg),
}

impl Instr {
    fn toggle(self) -> Self {
        use self::Instr::*;

        match self {
            Toggle(a) | Decrement(a) => Increment(a),
            Increment(a)    => Decrement(a),
            JmpNotZro(a, b) => Copy(a, b),
            Copy(a, b)      => JmpNotZro(a, b),
        }
    }
}

impl FromStr for Instr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(" ");
        let instr = words.next().unwrap(); // split always has at least 1 item, should never panic
        let a: Arg = words.next().ok_or("too few arguments".to_string())?.parse()?; //all instructions have one argument;
        let b: Result<Arg, _> = words.next().ok_or("too few arguments".to_string()).and_then(|s| s.parse());

        match instr {
            "cpy" => Ok(Instr::Copy(a, b?)),
            "inc" => Ok(Instr::Increment(a)),
            "dec" => Ok(Instr::Decrement(a)),
            "jnz" => Ok(Instr::JmpNotZro(a, b?)),
            "tgl" => Ok(Instr::Toggle(a)),
            s     => Err(format!("`{}`: invalid instruction.", s)),
        }
    }
}

pub fn adv_main(input: Vec<String>) {

    let mut input: Vec<Instr> = input.iter().map(|s| s.parse().unwrap() ).collect();
    let mut cpu = [12i32, 0i32, 0i32, 0i32];

    let mut pc: i32 = 0;
    let end = input.len() as i32;

    while pc < end {
        let instr = input[pc as usize];
        //println!("{:?}", instr);   

        match instr {
            Instr::Copy(v, Arg::Reg(r)) => cpu[ usize::from(r) ] = v.get_value(cpu),
            Instr::Increment(Arg::Reg(r)) => cpu[ usize::from(r) ] += 1,
            Instr::Decrement(Arg::Reg(r)) => cpu[ usize::from(r) ] -= 1,
            Instr::JmpNotZro(x, y) => {
                if x.get_value(cpu) != 0 {
                    pc += y.get_value(cpu);
                    continue;
                }
            },
            Instr::Toggle(v) => {
                input.get_mut((pc + v.get_value(cpu)) as usize).map(|v|{
                    *v = v.toggle();
                });
            },
            _   => {},
        }

        pc += 1;
    }

    println!("a: {}", cpu[0]);

}


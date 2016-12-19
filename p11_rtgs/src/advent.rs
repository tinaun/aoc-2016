/// advent of code 
///
/// problem 11 
///
/// elevator travel, dynamic programming
/// 

pub static PROBLEM_NUMBER: &'static str = "11"; 

use arrayvec::ArrayVec;
use itertools::Itertools;

use std::collections::HashSet;
use std::str::FromStr;

use self::Item::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Item {
    Generator(char),
    Microchip(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Elevator {
    floors: ArrayVec<[ ArrayVec<[Item; 32]>; 4]>,
    location: usize,
}

impl Item {
    fn unwrap(&self) -> char {
        match *self {
            Generator(c) => c,
            Microchip(c) => c,
        }
    }

    fn matches(&self, other: Self) -> bool {
        self.unwrap() == other.unwrap()
    }
}

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let element = s.chars().nth(0);
        let itemtype = s.chars().nth(1);

        match (element, itemtype) {
            (Some(e), Some('G')) => Ok(Generator(e)),
            (Some(e), Some('M')) => Ok(Microchip(e)),
            (Some(e), _)         => Err("invalid item type".to_string()),
            _                    => Err("invalid input".to_string()),
        }
    }
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            floors: ArrayVec::from([ArrayVec::new(), ArrayVec::new(), ArrayVec::new(), ArrayVec::new()]),
            location: 1,
        }
    }

    fn add_item(&mut self, floor: usize, item: Item) {
        self.floors[floor-1].push(item);
    }

    fn is_valid(&self) -> bool {
        if self.location > 1 && self.location < 5 {
            true

        } else {
            false
        }
    }


    fn valid_next(&self) -> Vec<Self> {

        let ref cfloor = self.floors[ self.location - 1 ];

        let next = cfloor.iter().combinations(1).chain( cfloor.iter().combinations(2) )
                         .cartesian_product( (0..1) ).count();

        println!("possible moves from position: {}", next);

        Vec::new()
    }

}

pub fn adv_main(input: Vec<String>) {
    
    let mut elevator = Elevator::new();

    for line in input {
        let words: Vec<_> = line.split(" ").collect();
        let floor: usize = words[0][..1].parse().unwrap();

        for item in &words[1..] {
            elevator.add_item(floor, item.parse().unwrap());
        }
    }

    println!("{:#?}", elevator);

    let _ = elevator.valid_next();
}

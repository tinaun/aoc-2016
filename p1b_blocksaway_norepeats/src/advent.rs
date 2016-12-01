/// advent of code 
/// 
/// problem 1 part 2
///
/// given a list of directions for navigating city blocks, 
/// find the first duplicated location and it's manhattan distance from the inital position

pub static PROBLEM_NUMBER: &'static str = "1b"; 

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn_left(&mut self) {
        use self::Direction::*;
        *self = match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        };
    }

    fn turn_right(&mut self) {
        use self::Direction::*;
        *self = match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        };
    }
}



pub fn adv_main(input: Vec<String>) {
    let directions: Vec<&str> = input[0].split(", ").collect();

    let mut location = (0, 0);

    let mut direction = Direction::North;
    let mut visited_corners: HashSet<(i32, i32)> = HashSet::new();

    'search: for dir in directions {
        println!("turn {} and go {} units.", &dir[0..1], &dir[1..]);

        match &dir[0..1] {
            "L" => direction.turn_left(),
            "R" => direction.turn_right(),
            _ => {},
        }

        let blocks: i32 = (&dir[1..]).parse().unwrap();

        for _ in 0..blocks {
            match direction {
                Direction::North => { location.0 += 1; },
                Direction::South => { location.0 -= 1; },
                Direction::East => { location.1 += 1; },
                Direction::West => { location.1 -= 1; },
            }

            if !visited_corners.insert(location) {
                break 'search;
            }
        }
    }

    println!("{} blocks north, {} blocks east - {} total.", 
             location.0, location.1, location.0.abs() + location.1.abs());


}

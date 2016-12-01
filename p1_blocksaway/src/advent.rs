/// advent of code 
///
/// problem 1 part 1
///
/// given a list of relative directions to navigate city blocks
/// find the manhattan distance to the final location

pub static PROBLEM_NUMBER: &'static str = "1a"; 


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

    let mut ns_blocks = 0;
    let mut ew_blocks = 0;

    let mut direction = Direction::North;

    for dir in directions {
        println!("turn {} and go {} units.", &dir[0..1], &dir[1..]);

        match &dir[0..1] {
            "L" => direction.turn_left(),
            "R" => direction.turn_right(),
            _ => {},
        }

        let blocks: i32 = (&dir[1..]).parse().unwrap();

        match direction {
            Direction::North => { ns_blocks += blocks; },
            Direction::South => { ns_blocks -= blocks; },
            Direction::East => { ew_blocks += blocks; },
            Direction::West => { ew_blocks -= blocks; },
        }
    }

    println!("{} blocks north, {} blocks east - {} total.", 
             ns_blocks, ew_blocks, ns_blocks.abs() + ew_blocks.abs());


}

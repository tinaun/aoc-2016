/// advent of code 
///
/// problem 13
///
/// pathfinding, with gif output
///

pub static PROBLEM_NUMBER: &'static str = "13"; 

use gif::{Frame, Encoder, Repeat, SetParameter};
use std::fs::File;
use std::io::Result;

static SIZE: u16           = 50;
static COLOR_MAP: [u8; 21] = [0xFF, 0xFF, 0xFF,  //empty floor
                              0x00, 0x00, 0x00,  //wall
                              0xFF, 0x00, 0x00,  //current
                              0x33, 0x33, 0x33,  //visited
                              0x00, 0x33, 0xFF,  //to visit
                              0x00, 0xFF, 0x00,  //goal
                              0xFF, 0x66, 0x66]; //path


use std::collections::HashMap;
use std::collections::HashSet;

type Position = (u64, u64);

fn init_gif(f: &mut File) -> Result<Encoder<&mut File>> {
    let mut e = Encoder::new(f, SIZE, SIZE, &COLOR_MAP[..])?;
    let _ = e.set(Repeat::Infinite)?;

    Ok(e)
}

fn create_frame<'a>(current: Position, goal: Position, distances: &'a HashMap<Position, i64>, 
                    visited: &'a HashSet<Position>, to_visit: &'a [(i64, Position)],
                    delay: u16, secret: u64) -> Frame<'a> {
    use std::borrow::Cow;

    let mut path: Vec<Position> = Vec::new();
    let mut cur = current;

    while *distances.get(&cur).unwrap() > 0 {
        let (_, next) = get_neighbors(cur, secret).iter().filter(|c| visited.contains(c))
                                             .map(|c| (*distances.get(c).unwrap(), *c))
                                             .min_by_key(|&(d, _)| d).unwrap();
        
        path.push(next);
        cur = next;
    }
    

    let mut buffer: Vec<u8> = Vec::new();

    for y in 0..SIZE {
        for x in 0..SIZE {
            let pos = (x as u64, y as u64);

            let next = if pos == current {
                2
            } else if pos == goal {
                5
            } else if path.contains(&pos) {
                6
            } else if visited.contains(&pos) {
                3
            } else if to_visit.iter().find(|&&(_, p)| p == pos).is_some() {
                4
            } else if !is_open(pos, secret) {
                1
            } else {
                0
            };


            buffer.push(next);
        }
    }


    let mut frame = Frame::default();
    frame.width = SIZE;
    frame.height = SIZE;
    frame.delay = delay;
    frame.buffer = Cow::Owned(buffer);

    frame
}

fn get_neighbors(p: Position, secret: u64) -> Vec<Position> {
    let (x, y) = p;

    let mut neighbors = vec![(x + 1, y), (x, y + 1)];

    if x > 0 {
        neighbors.push((x-1, y));
    }

    if y > 0 { 
        neighbors.push((x, y-1));
    }

    neighbors.into_iter().filter(|&p| is_open(p, secret)).collect()

}

fn is_open(p: Position, secret: u64) -> bool {
    let (x, y) = p;

    let sum = x*x + 3*x + 2*x*y + y + y*y + secret;
    
    format!("{:b}", sum).chars().filter(|&c| c == '1').count() % 2 == 0
}


pub fn adv_main(input: Vec<String>) {
    let mut image = File::create("output.gif").unwrap();
    let mut encoder = init_gif(&mut image).unwrap();
    
    let secret = input[0].parse().unwrap();
    let goal: Vec<_> = input[1].split(" ").collect();

    let initial = (1, 1);
    let goal = (goal[0].parse().unwrap(), goal[1].parse().unwrap());

    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();

    distances.insert(initial, 0);
    let mut current = initial;

    while !distances.contains_key(&goal) {
        let distance = *distances.get(&current).unwrap();
        visited.insert(current);

        for loc in get_neighbors(current, secret) {
            if visited.contains(&loc) {
                continue;
            }

            let dist = *distances.entry(loc).or_insert(distance + 1);
            if dist > distance + 1 {
                distances.insert(loc, distance + 1);
            }

            to_visit.push(( *distances.get(&loc).unwrap(), loc));
        }

        to_visit.sort_by_key(|&(d, (x, y))| {
            let x = x as i64 - goal.0 as i64;
            let y = y as i64 - goal.1 as i64;

            -( d /*+ (x*x + y*y) */ ) 
            
        });
       
        {   // <- i hate this
            let next_frame = create_frame(current, goal, &distances, &visited, &to_visit, 10, secret);
            encoder.write_frame(&next_frame).unwrap();
        }

        current = to_visit.pop().unwrap().1;
    }

    println!("the length to {:?} is {}.", goal, distances.get(&goal).unwrap());
    println!("{} visited locations 50 steps away.", distances.iter().filter(|&(_, &value)| value < 51).count());

    let next_frame = create_frame(goal, goal, &distances, &visited, &to_visit, 1000, secret);
    encoder.write_frame(&next_frame).unwrap();
}


/// advent of code 2016 
///
/// problem 24 - pathfind + traveling salesman
/// 

pub static PROBLEM_NUMBER: &'static str = "24"; 

use super::errors::*;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use permutohedron::LexicalPermutation;

type Position = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Item {
    Wall,
    Space,
    Point(u8),
}

impl Item {
    fn is_open(&self) -> bool {
        match *self {
            Item::Wall => false,
            _          => true,
        }
    }

    fn is_point(&self) -> bool {
        match *self {
            Item::Point(_) => true,
            _              => false,
        }
    }
}

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Item::Wall),
            "." => Ok(Item::Space),
            s   => match s.parse() {
                Ok(u) => Ok(Item::Point(u)),
                Err(_) => Err(format!("invalid character: {}", s)),
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Grid {
    points: usize,
    data: Vec<Vec<Item>>,
}

impl Grid {
    fn new(g: Vec<String>) -> Self {

        let mut points = 0;
        let data = g.into_iter().map(|s| {
            let l = s.len();
            s.split("").skip(1).take(l).map(|c| {
                let c: Item = c.parse().unwrap();
                if c.is_point() {
                    points += 1;
                }

                c 
            }).collect() 
        }).collect();


        Grid {
            points: points,
            data: data,
        }
    }

    fn get(&self, x: usize, y: usize) -> Item {
        self.data[y][x]
    }

    fn get_neighbors(&self, p: Position) -> Vec<Position> {
        let (x, y) = p;
        let mut neighbors = vec![(x + 1, y), (x, y + 1)];

        if x > 0 {
            neighbors.push((x-1, y));
        }
        if y > 0 { 
            neighbors.push((x, y-1));
        }

        neighbors.into_iter().filter( |&(x, y)| self.get(x, y).is_open() ).collect()
    }

    fn point_pos(&self, p: usize) -> Option<Position> {
        let p = p as u8;

        for y in 0 .. self.data.len() {
            for x in 0 .. self.data[0].len() {
                match self.get(x, y) {
                    Item::Point(q) if p == q => return Some((x, y)),
                    _                  => {},
                }
            }
        }

        None
    }
}

pub fn adv_main(input: Vec<String>) -> Result<(), Error> {
    let grid = Grid::new(input);

    let paths: Vec<Position> = (0..grid.points).tuple_combinations().collect();
    let mut path_distances: Vec<Vec<usize>> = vec![ vec![0 ; grid.points] ; grid.points ];

    for (x, y) in paths {
        //println!("{:?} {:?}", x, y);
        let initial = grid.point_pos(x).unwrap();
        let goal = grid.point_pos(y).unwrap();
        //println!("{:?} {:?}", initial, goal);

        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();

        distances.insert(initial, 0usize);
        let mut current = initial;

        while !distances.contains_key(&goal) {
            let distance = *distances.get(&current).unwrap();
            visited.insert(current);

            for loc in grid.get_neighbors(current) {
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
                let x = x as isize - goal.0 as isize;
                let y = y as isize - goal.1 as isize;

                -( d as isize + x.abs() + y.abs() ) 
                
            });

            current = to_visit.pop().unwrap().1;
        }

         path_distances[x][y] = *distances.get(&goal).unwrap();
         path_distances[y][x] = *distances.get(&goal).unwrap();
    }

    println!("path map:");
    for p in &path_distances {   
        println!("{:?}", p);
    }

    let mut distance: usize = 100000;
    let mut ordering: Vec<usize> = (1..grid.points).collect();
    let mut shortest: Vec<usize> = Vec::new();
   
    loop {
        let mut dist = path_distances[0][ ordering[0] ];

        for (x, y) in ordering.iter().cloned().tuple_windows() {
            dist += path_distances[x][y]; 
        }

        // remove for part 1
        dist += path_distances[ ordering[ordering.len() - 1] ][0];

        if distance > dist {
            distance = dist;
            shortest = ordering.clone();
        }

        if !ordering.next_permutation() {
            break;
        }
    }
    
    println!("shortest distance: 0 => {:?} => 0 - {}.", shortest, distance);
    Ok(())   
}



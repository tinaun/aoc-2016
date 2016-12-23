/// advent of code 2016 
///
/// problem 22
/// 

pub static PROBLEM_NUMBER: &'static str = "22"; 

use std::collections::HashMap;
use std::fmt::{self, Display};

use super::errors::*;
use regex::Regex;


lazy_static! {
    static ref CREATE_NODE: Regex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)").unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    location: (usize, usize),
    size: usize,
    used: usize,
}

impl Node {
    fn from_str(input: &str) -> Result<Self, Error> {
        CREATE_NODE.captures(input).and_then(|caps| {
            let caps: Vec<usize> = caps.iter().filter_map(|c| c.unwrap().parse().ok() ).collect();

            if caps.len() < 4 {
                None
            } else { 
                Some(Node {
                    location: (caps[0], caps[1]),
                    size: caps[2],
                    used: caps[3],
                })
            }
        }).ok_or(Error::ParseError)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GridItem {
    Item,
    Fixed,
    Empty,
    Goal,
}

struct Grid {
    empty: (usize, usize),
    height: usize,
    width: usize,
    data: Vec<Vec<GridItem>>,
}

impl Grid {
    fn from_nodes(mut nodes: Vec<Node>) -> Self {
        nodes.sort_by_key(|n| n.location);
        let last = nodes.pop().unwrap();
        println!("{:?}", last);
        let (width, height) = last.location;
        nodes.push(last);

        let mut grid = Grid {
            empty: (0, 0),
            height: height+1,
            width: width+1,
            data: vec![ vec![GridItem::Empty; width+1]; height+1],
        };
        
        for n in nodes {
            let (x, y) = n.location;
            grid.data[y][x] = if n.size > 100 {
                GridItem::Fixed
            } else if n.used == 0 {
                grid.empty = n.location;
                GridItem::Empty
            } else {
                GridItem::Item
            };
        }
        grid.data[0][width] = GridItem::Goal; 

        grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<String> = self.data.iter().map(|v| {
            v.iter().map(|gi|{
                match *gi {
                    GridItem::Empty => '_',
                    GridItem::Item => '.',
                    GridItem::Fixed => '#',
                    GridItem::Goal => 'G',
                }
            }).collect()
        }).collect();

        write!(f, "{}\n", s.join("\n")) 
    }
}

pub fn adv_main(input: Vec<String>) -> Result<(), Error> {

    let nodes: Vec<_> = input.into_iter().filter_map(|s| Node::from_str(&s).ok()).collect();

    let mut pairs = Vec::new();
    //let mut grid: HashMap<(usize, usize), Node> = HashMap::new();

    for a in &nodes {
        for b in &nodes {
            if a != b && a.used > 0 && a.used <= b.size - b.used {
                pairs.push((a.location,b.location))
            }
        }
    }

    println!("pairs: {}", pairs.len());

    let initial = Grid::from_nodes(nodes);
    println!("grid:\n{}", initial);

    Ok(())
}



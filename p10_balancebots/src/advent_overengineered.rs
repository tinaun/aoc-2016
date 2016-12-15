/// advent of code 
///
/// problem 7 
///
/// given a list of relative directions on a 3x3 keypad
/// find the passcode, ignoring impossible directions

pub static PROBLEM_NUMBER: &'static str = "4a"; 

use std::rc::{Rc, Weak};
use std::Cell::RefCell;
use std::collections::BTreeMap;


#[derive(Debug, Clone)]
struct Bot {
    pub id: usize, 

    values: [Option<usize>; 2],
    used: bool

    low: Weak<BotIndex>,
    high: Weak<BotIndex>,
}

type BotIndex = RefCell<Option<Bot>>;

impl Bot {
    fn new(id: usize, low: Weak<BotIndex>, high: Weak<BotIndex>) -> Self {
        Bot {
            id: id,

            values: [None; 2],
            used: false,

            low: low,
            high: high,
        }
    }

    fn add_value(&mut self, value: usize) {
        if self.values[0].is_none() {
            self.values[0] = value;
        } else if self.values[1].is_none() {
            self.values[1] = value;
        } else {
            panic!(format!("bot #{} has three values", self.id));
        }
    }

    fn is_full(&self) -> bool {
        self.values.iter().all(|&x| x.is_some())
    }

    fn exec(&mut self) {
        if self.is_full() && !self.used {
            let min = self.values.iter().min();
            let max = self.values.iter().max();

            let strong_low = self.low.upgrade();
            match strong_low {
                Some(o) => o.borrow_mut().map(|b| b.add_value(min)),
                None => println!("bot {} output - {}", self.id, max),
            }
            
            let strong_high = self.high.upgrade();
            match strong_low {
                Some(o) => o.borrow_mut().map(|b| b.add_value(min)),
                None => println!("bot {} output - {}", self.id, max),
            }
        }

        self.used = true;
    }
}

pub fn adv_main(input: Vec<String>) {
    let mut bot_list: BTreeMap<usize, Rc<BotIndex>> = BTreeMap::new();
    let mut input_list: Vec<(usize, usize)> = Vec::new();

    for line in input {
        let words: Vec<_> = line.split(" ").collect();

        match words[0] {
            "value" => {
                input_list.push( (words[1].parse().unwrap(), words[5].parse().unwrap()) );
            },
            "bot" => {
                let id = words[1].parse().unwrap()

                let left_index = if words[5] == "bot" {
                    let id = words[6].parse().unwrap();
                    let r = bot_list.entry(id).or_insert( Rc::new(RefCell::new(None)) ).clone();
                    r.downgrade()
                } else {
                    Weak::new()
                }
                
                let right_index = if words[10] == "bot" {
                    let id = words[11].parse().unwrap();
                    let r = bot_list.entry(id).or_insert( Rc::new(RefCell::new(None)) ).clone();
                    r.downgrade()
                } else {
                    Weak::new()
                }

                if !bot_list.contains_key(id) {


                }

                let bot = Bot::new(id, left_index, right_index);

                bot_list.insert( id, Bot::new(id, left_index, right_index)
            },
            _ => {}
        }
    }
}


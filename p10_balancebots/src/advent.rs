/// advent of code 
///
/// problem 10
/// balance bots

pub static PROBLEM_NUMBER: &'static str = "10"; 

use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum BotOutput {
    Bot(usize),
    Output(usize),
}

impl BotOutput {
    fn id(&self) -> usize {
        match *self {
            BotOutput::Bot(i) => i,
            BotOutput::Output(i) => i,
        }
    }

    fn is_bot(&self) -> bool {
        match *self {
            BotOutput::Bot(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    pub id: usize, 

    values: [Option<usize>; 2],
    used: bool,

    low: BotOutput,
    high: BotOutput,

    //factory: Factory,
} 

impl Bot {
    fn new(id: usize, low: BotOutput, high: BotOutput) -> Self {
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
            self.values[0] = Some(value);
        } else if self.values[1].is_none() {
            self.values[1] = Some(value);
        } else {
            panic!(format!("bot #{} has three values", self.id));
        }
    }

    fn is_full(&self) -> bool {
        self.values.iter().all(|&x| x.is_some())
    }

    fn exec(&mut self) -> Option<[(usize, BotOutput); 2]> {
        if self.is_full() && !self.used {
            let min = self.values.iter().min().unwrap().unwrap();
            let max = self.values.iter().max().unwrap().unwrap();

            self.used = true;
            Some([(min, self.low), (max, self.high)])
        } else {
            None
        }
    }
}

pub fn adv_main(input: Vec<String>) {
    let mut bot_list: BTreeMap<usize, Bot> = BTreeMap::new();
    
    let mut input_list: Vec<(usize, usize)> = Vec::new();
    let mut output_list: BTreeMap<usize, usize> = BTreeMap::new();

    for line in input {
        let words: Vec<_> = line.split(" ").collect();

        match words[0] {
            "value" => {
                input_list.push( (words[1].parse().unwrap(), words[5].parse().unwrap()) );
            },
            "bot" => {
                let id = words[1].parse().unwrap();
                let low = words[6].parse().unwrap();
                let low = match words[5] {
                    "output" => BotOutput::Output(low),
                    _ => BotOutput::Bot(low),
                };

                let high = words[11].parse().unwrap();
                let high = match words[10] {
                    "output" => BotOutput::Output(high),
                    _ => BotOutput::Bot(high),
                };
                
                bot_list.insert(id, Bot::new(id, low, high));
            },
            _ => {}
        }
    }

    for (val, botid) in input_list {
        let mut bot = bot_list.get_mut(&botid).unwrap();

        bot.add_value(val);
    }

    //println!("{:?}", bot_list);

    loop {
        let active_bots = { 
            let active_bots: Vec<_> = bot_list.iter_mut()
                                        .filter(|&(_, ref b)| b.is_full() && !b.used)
                                        .map(|(&id, b)| (id, b.exec().unwrap()) )
                                        .collect();

            active_bots
        };

        if active_bots.len() == 0 {
            break;
        } 

        for (id, result) in active_bots {
            for &(value, output) in result.iter() {
                if output.is_bot() {
                    let mut bot = bot_list.get_mut(&output.id()).unwrap();
                    bot.add_value(value);

                } else {
                    println!("bot {} put {} to output {}.", id, value, output.id()); 
                    output_list.insert(output.id(), value);
                }
            }
        }

    }

    for (id, bot) in bot_list {
        println!("bot {} has values {:?}", id, bot.values);
    }

    println!("output_0 * output_1 * output_2 = {}.", output_list.iter().take(3).fold(1, |acc, (_, d)| {
        acc * d
    }));

}


/// advent of code 
///
/// problem 8
///
/// generate password by drawing on a display
/// fancy terminal output

pub static PROBLEM_NUMBER: &'static str = "8"; 

use std::cmp::{min};
use std::fmt;
use std::thread;
use std::time::Duration;

use term;

#[derive(Debug)]
struct Display {
    pub width: usize,
    pub height: usize,
    data: Vec<bool>, 
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for x in 0..self.data.len() {
            if x % self.width == 0 { output.push('\n'); }
            
            let ch = if self.data[x] { '\u{2588}' } else { ' ' };
            output.push(ch);
        }

        write!(f, "{}", output)
    }
}

impl Display {
    fn new(width: usize, height: usize) -> Self {
        Display {
            width: width,
            height: height,
            data: vec![false; width * height],
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        let (width, height) = (min(width, self.width), min(height, self.height));

        for x in 0..height {
            for y in 0..width {
                self.data[self.width * x + y] = true;
            }
        }
    }

    fn rotate_col(&mut self, col: usize, amount: usize) {
        let col_vec: Vec<_> = self.data.iter().enumerate()
                                   .filter(|&(i, _)| i % self.width == col)
                                   .map(|(_, c)| c).cloned().collect();

        for x in 0 .. self.height {
            self.data[ (self.width * (x + amount) + col) % (self.width * self.height) ] = col_vec[x];
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        let row_vec: Vec<_> = self.data.iter().skip( self.width * row )
                                              .take( self.width )
                                              .cloned().collect();

        for y in 0 .. self.width {
            self.data[ self.width * row + ((y + amount) % self.width) ] = row_vec[y];
        }
    }

    fn get_lit(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }
}



pub fn adv_main(input: Vec<String>) {

    let mut screen = Display::new(50, 6);
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    //t.bg(term::color::RED).unwrap();
    writeln!(t, "{}", screen).unwrap();

    for line in input {
        let words: Vec<_> = line.split(" ").collect();

        match words[0] {
            "rect" => {
                let dimensions: Vec<usize> = words[1].split("x").map(|x| x.parse().unwrap()).collect();
                screen.rect(dimensions[0], dimensions[1]);
            },
            "rotate" => {
                let (dir, line) = words[2].split_at(2);
                let line = line.parse().unwrap();
                let shift = words[4].parse().unwrap();

                if dir.starts_with("x") {
                    screen.rotate_col(line, shift);
                } else {
                    screen.rotate_row(line, shift);
                }   
            },
            _ => {}
        }

        for _ in 0..(screen.height+1) {
            t.cursor_up().unwrap();
        }

        writeln!(t, "{}", screen).unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    t.reset().unwrap();
    println!("lit pixels: {}", screen.get_lit());

}


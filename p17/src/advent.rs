/// advent of code 2016 
///
/// problem 17
/// md5 maze

pub static PROBLEM_NUMBER: &'static str = "17"; 

use md5;
use std::io::Result;

#[derive(Debug)]
struct Path {
    movstr: String,
    location: (i64, i64),
}

impl Path {
    fn new(input: &str) -> Self {
        Path {
            movstr: input.to_string(),
            location: (0,0),
        }
    }

    fn is_valid(&self) -> bool {
        match self.location {
            (-1 , _) | (4 , _) | (_ , -1) | (_ , 4) => return false,
            _ => {},
        }

        let last = self.movstr.len() - 1;
        let digest = format!("{:x}", md5::compute(&self.movstr[..last].as_bytes())).into_bytes();
        let last = self.movstr.chars().nth(last).unwrap();

        //println!("{}", self.movstr);

        let key = match last {
            'U' => digest[0],
            'D' => digest[1],
            'L' => digest[2],
            _   => digest[3],
        };

        match key {
            b'b' ... b'f' => true,
            _           => false,
        }
    }

    fn get_next(&self) -> Vec<Self> {
        let dirs = ['U', 'D', 'L', 'R'];
        let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        
        dirs.iter().zip(deltas.iter()).map(|(&dir, delta)|{
            let mut newstr = self.movstr.clone();
            newstr.push(dir);

            Path {
                movstr: newstr,
                location: (self.location.0 + delta.0, self.location.1 + delta.1),
            }

        }).filter(|p| p.is_valid() ).collect()
        
    } 
}

pub fn adv_main(input: Vec<String>) -> Result<()> {
    let len = input[0].len();
    let initial = Path::new(&input[0]);

    let mut paths = vec![initial];

    while paths.len() > 0 {
        paths = paths.iter().flat_map(|p| p.get_next()).filter(|path| {
            if path.location == (3,3) {
                if path.movstr.len() - len < 40 {
                     println!("valid path found: {}", &path.movstr[len..]);
                } else {
                    println!("valid path found of length {}.", path.movstr.len() - len);
                }
                false 
            } else { true }
        }).collect();
        
    }

    println!("no more valid paths!");

    Ok(())
}


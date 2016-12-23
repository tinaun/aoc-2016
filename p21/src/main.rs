use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

mod advent;
mod errors {
    use std::string::FromUtf8Error;
    use std::num::ParseIntError;
    
    #[derive(Debug)]
    pub enum Error {
        ParseError,
        StrError(String),
    }

    impl From<ParseIntError> for Error {
        fn from(_: ParseIntError) -> Self {
            Error::ParseError
        }
    }

    impl From<FromUtf8Error> for Error {
        fn from(_: FromUtf8Error) -> Self {
            Error::ParseError
        }
    }

    impl<'a> From<&'a str> for Error {
        fn from(s: &str) -> Self {
            Error::StrError(s.to_string())
        }
    }
    
}


fn open_file(filename: &str) -> io::Result<Vec<String>> {
    let f = try!(File::open(filename));
    let f = BufReader::new(f);

    let mut lines = Vec::new();

    for line in f.lines() {
        let line = try!(line);
        lines.push(line);
    }

    Ok(lines)
}

fn main() {
    use std::env;
    
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("usage: {} [file] - performs advent of code 2016 problem #{} on input file.", 
                  args[0], advent::PROBLEM_NUMBER);
        return;
    }

    let lines = open_file(&args[1]).unwrap();
    
    match advent::adv_main(lines) {
        Ok(_) => {},
        Err(e) => {
            println!("error: {:?}", e);
            std::process::exit(1);
        }
    }
}

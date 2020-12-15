use std::{collections::HashMap, env, error::Error, str::FromStr};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Convenience macro for generating errors
macro_rules! error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Box::<dyn Error>::from(res)
    }};
}

#[derive(Debug, Clone, Copy)]
struct Line {
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Memory {
}

// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() -> Result<(), Box<dyn Error>> {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Open file
    let path = args.get(1).ok_or(error!("Usage: cargo run -- <input file>"))?;
    let file = File::open(path)?;
    
    // Read file
    //let _lines = BufReader::new(file).lines().map(|s| s.unwrap().parse().unwrap()).collect::<Vec<Line>>();

    // Map of number -> last pos
    let mut numbers = HashMap::<u32, u32>::new();
    numbers.insert(1, 0);
    numbers.insert(0, 1);
    numbers.insert(16, 2);
    numbers.insert(5, 3);
    numbers.insert(17, 4);

    let mut next = 4;
    let mut next_pos = 5;
    while next_pos < 30000000 - 1 {
        if let Some(&last_pos) = numbers.get(&next) {
            numbers.insert(next, next_pos);
            next = next_pos - last_pos;
        }
        else {
            numbers.insert(next, next_pos);
            next = 0;
        }
        next_pos += 1
    }

    println!("{:?}", next);

    // Everything ok!
    Ok(())
}

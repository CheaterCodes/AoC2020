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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Line {
    Mask(u64, u64, u64),
    Write(u64, u64)
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        match parts.next() {
            Some("mask") => {
                let mask = parts.next().ok_or(error!("Missing value!"))?;
                let set_mask = u64::from_str_radix(mask.replace("X", "0").as_str(), 2)?;
                let clear_mask = u64::from_str_radix(mask.replace("1", "X").replace("0", "1").replace("X", "0").as_str(), 2)?;
                let float_mask = u64::from_str_radix(mask.replace("1", "0").replace("X", "1").as_str(), 2)?;
                Ok(Line::Mask(set_mask, clear_mask, float_mask))
            },
            Some(s) if s.starts_with("mem[") => {
                let val = &s[4..s.len() - 1];
                Ok(Line::Write(val.parse()?, parts.next().ok_or(error!("Missing value!"))?.parse()?))
            },
            _ => Err(error!("Error paring line!"))
        }
    }
}

struct Memory {
    memory: HashMap<u64, u64>,
    set_mask: u64,
    clear_mask: u64,
    float_mask: u64
}

impl Memory {
    fn new() -> Self {
        Memory {
            memory: HashMap::new(),
            set_mask: 0,
            clear_mask: 0,
            float_mask: 0
        }
    }

    fn apply(&mut self, line: &Line) {
        match line {
            Line::Mask(set, clear, float) => {
                self.set_mask = *set;
                self.clear_mask = *clear;
                self.float_mask = *float;
            }
            Line::Write(address, value) => self.write(*address, *value)
        }
    }

    fn write(&mut self, address: u64, value: u64) {
        let value = (value | self.set_mask) & !self.clear_mask;
        self.memory.insert(address, value);
    }

    fn apply_correct(&mut self, line: &Line) {
        match line {
            Line::Mask(set, clear, float) => {
                self.set_mask = *set;
                self.clear_mask = *clear;
                self.float_mask = *float;
            }
            Line::Write(address, value) => {
                self.write_correct(*address, *value, self.set_mask, self.float_mask);
            }
        }
    }

    fn write_correct(&mut self, address: u64, value: u64, set_mask: u64, float_mask: u64) {
        if float_mask == 0 {
            self.memory.insert(address | set_mask, value);
        }
        else {
            // Bit ops commence!
            let last_float = 1 << u64::trailing_zeros(float_mask);
            // recurse with bit set and bit cleared
            self.write_correct(address | last_float, value, set_mask, float_mask & !last_float);
            self.write_correct(address & !last_float, value, set_mask, float_mask & !last_float);
        }
    }

    fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() -> Result<(), Box<dyn Error>> {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Open file
    let path = args.get(1).ok_or(error!("Usage: cargo run -- <input file>"))?;
    let file = File::open(path)?;
    
    // Read file
    let lines = BufReader::new(file).lines().map(|s| s.unwrap().parse().unwrap()).collect::<Vec<Line>>();

    let mut mem = Memory::new();
    for line in &lines {
        mem.apply(line);
    }

    println!("Total sum: {}", mem.sum());

    
    let mut mem = Memory::new();
    for line in &lines {
        mem.apply_correct(line);
    }

    println!("Total correct sum: {}", mem.sum());

    // Everything ok!
    Ok(())
}

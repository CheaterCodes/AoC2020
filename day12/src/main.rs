use std::{env, error::Error, str::FromStr};
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn(&self, value: i32) -> Self {
        let turns = ((value / 90) % 4 + 4) as usize;
        self.right_n(turns)
    }

    fn right_n(&self, n: usize) -> Self {
        if n == 0 {
            *self
        }
        else {
            self.right().right_n(n - 1)
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32
}

#[derive(Debug)]
struct Waypoint {
    north: i32,
    east: i32
}

impl Waypoint {
    fn turn(&mut self, value: i32) {
        let turns = ((value / 90) % 4 + 4) as usize;
        self.right_n(turns);
    }

    fn right_n(&mut self, n: usize) {
        if n > 0 {
            self.right();
            self.right_n(n - 1);
        }
    }

    fn right(&mut self) {
        let temp = self.north;
        self.north = -self.east;
        self.east = temp;
    }
}

#[derive(Debug)]
struct Ferry {
    north: i32,
    east: i32,
    dir: Direction,
    waypoint: Waypoint
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            dir: Direction::East,
            east: 0,
            north: 0,
            waypoint: Waypoint {
                east: 10,
                north: 1
            }
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => self.north += instruction.value,
            Action::South => self.north -= instruction.value,
            Action::East => self.east += instruction.value,
            Action::West => self.east -= instruction.value,
            Action::Left => self.dir = self.dir.turn(-instruction.value),
            Action::Right => self.dir = self.dir.turn(instruction.value),
            Action::Forward => match self.dir {
                Direction::North => self.north += instruction.value,
                Direction::South => self.north -= instruction.value,
                Direction::East => self.east += instruction.value,
                Direction::West => self.east -= instruction.value,
            }
        }
    }

    fn apply_correct(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::North => self.waypoint.north += instruction.value,
            Action::South => self.waypoint.north -= instruction.value,
            Action::East => self.waypoint.east += instruction.value,
            Action::West => self.waypoint.east -= instruction.value,
            Action::Left => self.waypoint.turn(-instruction.value),
            Action::Right => self.waypoint.turn(instruction.value),
            Action::Forward => self.forward(instruction.value)
        }
    }

    fn forward(&mut self, value: i32) {
        self.north += self.waypoint.north * value;
        self.east += self.waypoint.east * value;
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.get(..1) {
            Some("N") => Action::North,
            Some("S") => Action::South,
            Some("E") => Action::East,
            Some("W") => Action::West,
            Some("L") => Action::Left,
            Some("R") => Action::Right,
            Some("F") => Action::Forward,
            _ => return Err(error!("Error parsing action!"))
        };

        let value = s.get(1..).ok_or(error!("No value given!"))?.parse()?;

        Ok(Instruction {
            action: action,
            value: value
        })
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
    let instructions: Vec<Instruction> = BufReader::new(file).lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let mut ferry = Ferry::new();
    for instruction in &instructions {
        ferry.apply(instruction);
    }

    println!("Ferry: {:?}", ferry);
    println!("Manhatten Distance: {}", ferry.north.abs() + ferry.east.abs());
    
    let mut ferry = Ferry::new();
    for instruction in &instructions {
        ferry.apply_correct(instruction);
    }

    println!("Correct Ferry: {:?}", ferry);
    println!("Correct Manhatten Distance: {}", ferry.north.abs() + ferry.east.abs());

    // Everything ok!
    Ok(())
}

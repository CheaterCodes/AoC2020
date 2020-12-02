use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Check argument count
    if args.len() != 2 {
        println!("Usage: day1 <input file>");
        return;
    }

    // Open file
    let path = args.get(1).unwrap();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Problem opening file: {:?}", e)
    };
    
    // Parse file into numbers
    let lines: Vec<Line> = BufReader::new(file).lines().map(|l| l.unwrap()).map(|s| parse_line(s).expect("Error parsing line!")).collect();

    let mut valid_count = 0;
    for line in &lines {
        let char_count = line.password.chars().filter(|c| c == &line.letter).count();
        if line.min <= char_count && char_count <= line.max {
            valid_count += 1;
        }
    }

    println!("Valid passwords (Part one): {}", valid_count);
    
    let mut valid_count = 0;
    for line in &lines {
        let min = line.password.chars().nth(line.min - 1).unwrap_or_default() == line.letter;
        let max = line.password.chars().nth(line.max - 1).unwrap_or_default() == line.letter;
        if (min || max) && (!min || !max) {
            valid_count += 1;
        }
    }

    println!("Valid passwords (Part two): {}", valid_count);
}

struct Line {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

fn parse_line(s: String) -> Option<Line> {
    let parts: Vec<&str> = s.split(": ").collect();
    let password = parts.last().unwrap();
    let parts: Vec<&str> = parts.first().unwrap().split(" ").collect();
    let letter = parts.last().unwrap().parse::<char>().unwrap();
    let parts: Vec<&str> = parts.first().unwrap().split("-").collect();
    let max = parts.last().unwrap().parse::<usize>().unwrap();
    let min = parts.first().unwrap().parse::<usize>().unwrap();

    Some(Line {
        min: min,
        max: max,
        letter: letter,
        password: String::from(*password)
    })
}

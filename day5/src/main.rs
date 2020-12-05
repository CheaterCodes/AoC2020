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
    
    // Read file
    let mut input = String::new();
    let _ = BufReader::new(file).read_to_string(&mut input);

    // Parse seats
    let mut seats: Vec<usize> = input.split('\n').map(|line| line.replace('F', "0").replace('B', "1").replace('L', "0").replace('R', "1")).map(|bits| usize::from_str_radix(&bits, 2).unwrap()).collect();
    seats.sort();

    println!("Lowest seat ID: {}", seats.first().unwrap());
    println!("Highest seat ID: {}", seats.last().unwrap());

    let first = seats.first().unwrap();
    let my_seat = seats.iter().enumerate().find(|e| e.0 < e.1 - first).unwrap().1 - 1;
    println!("My seat ID: {}", my_seat);
}

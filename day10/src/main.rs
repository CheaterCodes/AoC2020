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
    let lines = BufReader::new(file).lines();
    let mut numbers: Vec<usize> = lines.map(|line| line.unwrap().parse().unwrap()).collect();
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let mut one = 0;
    let mut three = 0;

    let mut last = 0;
    for &n in &numbers {
        match n - last {
            1 => one += 1,
            2 => (),
            3 => three += 1,
            _ => unreachable!()
        }
        last = n;
    }
    println!("Part One: {}", one * three);

    let mut variants: Vec<u64> = vec![0; *numbers.last().unwrap() + 1];
    let len = variants.len();
    variants[len - 1] = 1;

    for &num in numbers.iter().rev().skip(1).chain(std::iter::once(&0)) {
        variants[num] += variants[num + 1];
        variants[num] += variants[num + 2];
        variants[num] += variants[num + 3];
    }

    println!("Total Variants: {:?}", variants[0]);
}

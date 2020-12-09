use std::{collections::HashSet, env};
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

    // Parse instructions
    let inst: Vec<(&str, i32)> = input.split("\n").map(|line| line.splitn(2, " ").collect::<Vec<&str>>()).map(|p| (p[0], p[1].parse().unwrap())).collect();

    let (_, acc) = check_inst(&inst);
    println!("Failed at acc: {}", acc);

    for change in 0..inst.len() {
        let mut fixed = inst.clone();
        match inst[change].0 {
            "jmp" => fixed[change].0 = "nop",
            "nop" => fixed[change].0 = "jmp",
            _ => ()
        }
        if let (true, acc) = check_inst(&fixed) {
            println!("Success! Acc: {}", acc);
        }
    }
}

fn check_inst(inst: &Vec<(&str, i32)>) -> (bool, i32) {
    let mut processed_inst = HashSet::<i32>::new();

    let mut acc = 0;
    let mut pc = 0;
    while processed_inst.insert(pc) {
        let cur_inst = inst[pc as usize];
        match cur_inst {
            ("nop", _) => {
                pc += 1
            },
            ("jmp", val) => {
                pc += val;
            },
            ("acc", val) => {
                acc += val;
                pc += 1;
            },
            _ => panic!()
        }

        if pc == inst.len() as i32 {
            return (true, acc);
        }
    }

    (false, acc)
}
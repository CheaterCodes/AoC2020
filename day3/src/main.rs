use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::{FromStr, ParseBoolError};

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
    let lines: Vec<Line> = BufReader::new(file).lines().map(|l| l.unwrap()).map(|s| s.parse::<Line>().unwrap()).collect();

    let trees = count_trees(&lines, 3, 1);
    println!("Trees found: {}", trees);
    
    let slope_1_1 = count_trees(&lines, 1, 1);
    let slope_3_1 = count_trees(&lines, 3, 1);
    let slope_5_1 = count_trees(&lines, 5, 1);
    let slope_7_1 = count_trees(&lines, 7, 1);
    let slope_1_2 = count_trees(&lines, 1, 2);

    println!("Other slopes: {}, {}, {}, {}, {}", slope_1_1, slope_3_1, slope_5_1, slope_7_1, slope_1_2);
    println!("Product: {}", slope_1_1 * slope_3_1 * slope_5_1 * slope_7_1 * slope_1_2);
}

fn count_trees(lines: &Vec<Line>, step_col: usize, step_row: usize) -> u64 {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;

    loop {
        col += step_col;
        if col >= lines[row].trees.len() {
            col -= lines[row].trees.len();
        }

        row += step_row;
        if lines[row].trees[col] {
            trees += 1;
        }

        if row == lines.len() - 1 {
            return trees;
        }
    }
}

struct Line {
    trees: Vec<bool>
}

impl<'a> FromStr for Line {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line{trees: s.chars().map(|c| c == '#').collect()})
    }
}


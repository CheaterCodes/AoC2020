use std::{env, error::Error};
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



// Build and run with `cargo run -- ./input.txt` from the day1 base dir
fn main() -> Result<(), Box<dyn Error>> {
    // Get commandline arguments
    let args: Vec<String> = env::args().collect();

    // Open file
    let path = args.get(1).ok_or(error!("Usage: cargo run -- <input file>"))?;
    let file = File::open(path)?;
    
    // Read file
    let initial_grid: Vec<Vec<char>> = BufReader::new(file).lines().map(|line| line.unwrap().chars().collect()).collect();

    // Part One
    let mut current_grid = initial_grid.clone();
    loop {
        let new_grid = iter_grid(&current_grid, 3, Some(1));
        if new_grid == current_grid {
            break;
        }
        current_grid = new_grid;
    }

    // Part Two
    let count = current_grid.iter().flatten().filter(|&&c| c == '#').count();
    println!("Occupied seats: {}", count);

    let mut current_grid = initial_grid.clone();
    loop {
        let new_grid = iter_grid(&current_grid, 4, None);
        if new_grid == current_grid {
            break;
        }
        current_grid = new_grid;
    }

    let count = current_grid.iter().flatten().filter(|&&c| c == '#').count();
    println!("Occupied seats (Part Two): {}", count);

    // Everything ok!
    Ok(())
}

fn iter_grid(grid: &Vec<Vec<char>>, max_neighbors: usize, max_dist: Option<usize>) -> Vec<Vec<char>> {
    (0..grid.len()).map(|row| (0..grid[row].len()).map(|col| iter_seat(grid, row, col, max_neighbors, max_dist)).collect()).collect()
}

fn iter_seat(grid: &Vec<Vec<char>>, row: usize, col: usize, max_neighbors: usize, max_dist: Option<usize>) -> char {
    let neighbors = count_neighbors_part_2(&grid, row, col, max_dist);
    match grid[row][col] {
        'L' if neighbors == 0 => '#',
        '#' if neighbors > max_neighbors => 'L',
        _ => grid[row][col]
    }
}

fn count_neighbors_part_2(grid: &Vec<Vec<char>>, row: usize, column: usize, max_dist: Option<usize>) -> usize {
    let mut count = 0;

    if dist_seat_occupied(grid, row, column, 0, 1)
        .map_or(false, |d| max_dist.map_or(true, |inner| d <= inner)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, 1, 1)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, 1, 0)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, 1, -1)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, 0, -1)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, -1, -1)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, -1, 0)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    if dist_seat_occupied(grid, row, column, -1, 1)
        .map_or(false, |dist| max_dist.map_or(true, |max_dist| dist <= max_dist)) {
        count += 1;
    }

    count
}

fn dist_seat_occupied(grid: &Vec<Vec<char>>, row: usize, column: usize, step_row: i32, step_col: i32) -> Option<usize> {
    let mut cur_row = row as i32;
    let mut cur_col = column as i32;
    let mut distance = 0;
    loop {
        cur_row += step_row;
        cur_col += step_col;
        distance += 1;
        if cur_col < 0 || cur_row < 0 {
            return None;
        }
        match grid.get(cur_row as usize).and_then(|row| row.get(cur_col as usize)) {
            Some('.') => continue,
            Some('#') => return Some(distance),
            _ => return None
        }
    }
}

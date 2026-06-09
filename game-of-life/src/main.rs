/*
https://codingdojo.org/kata/GameOfLife/

Game of life

This was presented as a PreparedKata at XP2005, by Emmanuel Gaillot.
Difficulty - medium

Problem Description
This Kata is about calculating the next generation of Conway’s game of life, given any starting position. See http://en.wikipedia.org/wiki/Conway%27s_Game_of_Life for background.

You start with a two dimensional grid of cells, where each cell is either alive or dead. In this version of the problem, the grid is finite, and no life can exist off the edges. 
When calculating the next generation of the grid, follow these rules:

   1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
   2. Any live cell with more than three live neighbours dies, as if by overcrowding.
   3. Any live cell with two or three live neighbours lives on to the next generation.
   4. Any dead cell with exactly three live neighbours becomes a live cell.

You should write a program that can accept an arbitrary grid of cells, and will output a similar grid showing the next generation.
*/

use std::{thread, time, vec};
mod patterndata;
mod userinput;

fn time_in_millis(millis:u64) -> time::Duration {return time::Duration::from_millis(millis);}

fn pretty_print_grid(width:usize, height:usize, grid:Vec<Vec<bool>>) {
    for i in 0..height {
        for j in 0..width {
            print!("{}",  if grid[i][j] {'#'} else {'.'});
        }
        println!();
    }
}

fn calculate_next_gen(width:usize, height:usize, grid:Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next_grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let neighbors_coords_deltas:[[usize; 2]; 8] = [[height-1, width-1], [height-1, 0], [height-1, 1], [0, width-1], [0, 1], [1, width-1], [1, 0], [1, 1]]; // "width-1" or "height-1" is the same as "-1" because edges just wrap around
    for i in 0..height {
        for j in 0..width {
            let mut alive_neighbors = 0;
            for delta in neighbors_coords_deltas {
                if grid[(i+delta[0])%height][(j+delta[1])%width] { // alive neighbor (wraps around edge)
                    alive_neighbors += 1;
                }
            }
            // if alive + (2 or 3 neighbors) = alive
            // or if dead + (exactly 3 neighbors) = alive
            if alive_neighbors == 3 || (grid[i][j] && alive_neighbors == 2) {
                next_grid[i][j] = true;
            }
            // everything else = dead (so keep as is because initialized at false)
        }
    }
    return next_grid;
}

fn main() {
    let width = userinput::user_input(1000, "Please enter the grid's width :");
    let height = userinput::user_input(1000, "Please enter the height :");
    
    let mut start_pattern_string = String::from("As starting grid, you can choose from these patterns :\n");
    for (i, pattern) in patterndata::PATTERNS.iter().enumerate() {
        start_pattern_string.push_str(&format!("  {}. {} ({}x{})\n", i, pattern.name, pattern.width, pattern.height).to_string());
    }
    start_pattern_string.push_str("Please select a starting pattern :");
    let mut start_pattern = 0;
    let mut valid_pattern = false;
    while !valid_pattern {
        start_pattern = userinput::user_input(patterndata::PATTERNS.len()-1, &start_pattern_string);
        if patterndata::PATTERNS[start_pattern].width > width {
            println!("Entered width of {} does not suppport pattern's width of {}. Please select another pattern.", width, patterndata::PATTERNS[start_pattern].width);
        } else if patterndata::PATTERNS[start_pattern].height > height {
            println!("Entered height of {} does not suppport pattern's height of {}. Please select another pattern.", height, patterndata::PATTERNS[start_pattern].height);
        } else {
            valid_pattern = true;
        }
    }

    // initialize with pattern in top left and false everywhere else
    let pattern = &patterndata::PATTERNS[start_pattern];
    let mut grid:Vec<Vec<bool>> = Vec::new();
    for i in 0..height {
        if i < pattern.height {
            let mut row = Vec::from(pattern.data[i]);
            row.extend(vec![false; width-pattern.width]);
            grid.push(row);
        } else {
            grid.push(vec![false; width]);
        }
    }

    let mut gen_number = 1;
    loop {
        println!("\n\n\n\n\n\n\n\n Generation {}", gen_number);
        pretty_print_grid(width, height, grid.clone());
        grid = calculate_next_gen(width, height, grid.clone());
        gen_number += 1;
        thread::sleep(time_in_millis(150));
    }
}

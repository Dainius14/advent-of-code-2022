use std::{io::{self, BufRead}, fs};

use advent_of_calendar_2022::day5_stacks::Stacks;

fn main() {
    let file = fs::File::open("data/day5_input.txt").unwrap();
    let reader = io::BufReader::new(file);
    
    let mut stack_lines = Vec::new();
    let mut move_lines = Vec::new();
    let mut reading_moves = false;
    for line in reader.lines() {
        let line_str = line.unwrap();

        if !reading_moves {
            if line_str.is_empty() {
                reading_moves = true;
            }
            else {
                stack_lines.push(line_str);
            }
        }
        else if !line_str.is_empty() {
            move_lines.push(line_str);
        }
    }

    let mut stacks = Stacks::from_str_lines(&stack_lines).unwrap();

    for line in move_lines {
        let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
        let x: usize = split_line.get(1).unwrap().parse().unwrap();
        let a: usize = split_line.get(3).unwrap().parse().unwrap();
        let b: usize = split_line.get(5).unwrap().parse().unwrap();
        
        stacks.move_x_from_a_to_b(x, a, b);
    }

    let joined_crates: String = stacks.get_last_crates().into_iter().collect();

    println!("{}", joined_crates);
}

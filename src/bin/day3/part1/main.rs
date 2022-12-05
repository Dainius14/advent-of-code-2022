use std::{fs, io::{self, BufRead}, collections::HashSet};

fn main() {
    let file = fs::File::open("data/day3_input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut duplicates: Vec<char> = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let (left, right) = line_str.split_at(line_str.len() / 2);
        
        let left_chars: Vec<char> = left.chars().collect();
        let right_chars: Vec<char> = right.chars().collect();

        let mut current_unique_types: HashSet<char> = HashSet::new();
        for left_char in left_chars {
            if right_chars.contains(&left_char) {
                current_unique_types.insert(left_char);
            }
        }

        duplicates.extend(current_unique_types.iter());
    }

    let mut total_score = 0;
    for duplicate in duplicates {
        total_score += if duplicate.is_lowercase() { (duplicate as i32) - 96 } else { (duplicate as i32) - 64 + 26 };
    }

    println!("Total score: {total_score}")

}
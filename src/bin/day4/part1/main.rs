use std::{fs, io::{self, BufRead}, str::FromStr};
use advent_of_calendar_2022::day4_section::Section;

fn main() {
    let file = fs::File::open("data/day4_input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut overlapping = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let comma_index = line_str.find(',').unwrap();

        let (left, right) = line_str.split_at(comma_index);
        let left_section = Section::from_str(left).unwrap();
        let right_section = Section::from_str(&right[1..]).unwrap();

        let overlap = left_section.overlaps_with(&right_section) || right_section.overlaps_with(&left_section);

        if overlap {
            overlapping += 1;
        }
    }

    println!("Overlapping: {overlapping}")
}

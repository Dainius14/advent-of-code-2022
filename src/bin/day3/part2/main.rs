use std::{fs, io::{self, BufRead}, collections::HashSet};

fn main() {
    let file = fs::File::open("data/day3_input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut unique_types: Vec<char> = Vec::new();
    let mut group_unique_types: HashSet<char> = HashSet::new();
    for (i, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();

        group_unique_types = if (i + 1) % 3 == 1 {
            line_str.chars().collect()
        } else {
            group_unique_types.iter()
            .filter(|&&x| line_str.contains(x))
            .cloned()
            .collect()
        };

        if (i + 1) % 3 == 0 {
            unique_types.extend(group_unique_types.iter());
            group_unique_types.clear();
        }
    }

    let mut priority_sum = 0;
    for unique_type in unique_types {
        priority_sum += type_to_priority(unique_type);
    }

    println!("Priority sum: {priority_sum}")

}

fn type_to_priority(unique_type: char) -> i32 {
    if unique_type.is_lowercase() { (unique_type as i32) - 96 } else { (unique_type as i32) - 64 + 26 }
}
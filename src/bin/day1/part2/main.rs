use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day1_input.txt")
        .expect("Should have been able to read file");
    
    let lines = contents.split('\n');

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for line in lines {
        let trimmed_line = line.trim_end();

        if trimmed_line.is_empty() {
            elf_calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        
        let calories: i32 = trimmed_line.parse().unwrap();
        current_calories += calories;
    }

    elf_calories.sort();
    elf_calories.reverse();
    let top3 = elf_calories[0..3].to_vec();
    let top3_sum: i32 = top3.iter().sum();

    println!("Top 3 sum: {top3_sum}");
}

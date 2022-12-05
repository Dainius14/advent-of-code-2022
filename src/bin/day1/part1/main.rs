use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day1_input.txt")
        .expect("Should have been able to read file");
    
    let lines = contents.split('\n');

    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in lines {
        let trimmed_line = line.trim_end();

        if trimmed_line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;

            continue;
        }
        
        let calories: i32 = trimmed_line.parse().unwrap();
        current_calories += calories;
    }

    println!("Max: {max_calories}");
}

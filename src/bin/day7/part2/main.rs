use advent_of_calendar_2022::{file_reader::read_lines_from_single_block, day7::Terminal};

fn main() {
    let lines = read_lines_from_single_block("data/day7_input.txt");

    let mut terminal = Terminal::default();
    for line in lines {
        _ = match try_get_command(&line) {
            Some(command) => terminal.execute_command(command),
            None => terminal.feed_command_result(&line), 
        }
    }

    terminal.execute_command("cd /").unwrap();
    let root = terminal.get_current_dir();
    let dir_sizes = terminal.get_directory_sizes_from_selected_dir(root);
    let mut sizes: Vec<&usize> = dir_sizes.values().collect();
    sizes.sort();

    let used_space = **(sizes.iter().max().unwrap());
    let free_space_left = 70_000_000 - used_space;
    let free_space_needed = 30_000_000 - free_space_left;

    let deleted_dir_size = sizes.iter().find(|x| ***x > free_space_needed).unwrap();

    println!("Deleted dir size: {}", deleted_dir_size);
}

fn try_get_command(line: &str) -> Option<&str> {
    if !line.starts_with('$') {
        return None;
    }
    Some(&line[2..])
}
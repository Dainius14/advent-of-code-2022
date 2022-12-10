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
    let sizes = terminal.get_directory_sizes_from_selected_dir(root);

    let sum: usize = sizes.values().filter(|size| size <= &&100_000_usize).sum();

    println!("Sum of such directories: {}", sum);
}

fn try_get_command(line: &str) -> Option<&str> {
    if !line.starts_with('$') {
        return None;
    }
    Some(&line[2..])
}


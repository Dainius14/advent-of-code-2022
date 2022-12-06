use advent_of_calendar_2022::file_reader::read_line_from_single_block;

fn main() {
    let line = read_line_from_single_block("data/day6_input.txt");

    const SIGNAL_LENGTH: usize = 4;

    for i in SIGNAL_LENGTH..line.len() {
        let start_pos = i - SIGNAL_LENGTH;
        let line_slice = &line[start_pos..i];

        let mut bytes: Vec<u8> = line_slice.bytes().collect();
        bytes.sort();
        bytes.dedup();
        if bytes.len() == SIGNAL_LENGTH {
            println!("Index: {}", i);
            break;
        }
    }
}
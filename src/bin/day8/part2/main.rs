use advent_of_calendar_2022::{day8::HeightMap, file_reader::read_lines_from_single_block};

fn main() {
    let lines = read_lines_from_single_block("data/day8_input.txt");
    let height_map = HeightMap::create_from_lines(&lines);
    let hightest_score =    height_map.get_highest_scenic_score();
    println!("Highest scenic score: {}", hightest_score);
}
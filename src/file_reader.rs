use std::{fs, io::{self, BufRead}};


pub fn read_blocks(file_name: &str) -> Vec<FileBlock> {
    let file = fs::File::open(file_name).expect("file to exist");
    let reader = io::BufReader::new(file);
    
    let mut blocks: Vec<FileBlock> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();

        if !line_str.is_empty() {
            lines.push(line_str);
            continue;
        }

        let block = FileBlock { lines };

        blocks.push(block);
        lines = Vec::new();
    }

    // There was only a single block
    if blocks.is_empty() {
        let block = FileBlock { lines };
        blocks.push(block);
    }

    blocks
}

pub fn read_lines_from_single_block(file_name: &str) -> Vec<String> {
    let blocks = read_blocks(file_name);

    blocks.first().unwrap().lines.to_owned()
}

pub fn read_line_from_single_block(file_name: &str) -> String {
    let lines = read_lines_from_single_block(file_name);

    lines.first().unwrap().to_owned()
}
pub struct FileBlock {
    pub lines: Vec<String>,
}
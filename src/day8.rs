use std::cmp;

pub struct HeightMap {
    map: Vec<Vec<usize>>,
}

impl HeightMap {
    pub fn create_from_lines(lines: &Vec<String>) -> HeightMap {
        let height = lines.len();

        let mut map = Vec::with_capacity(height);
        
        for line in lines {
            let row: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
            map.push(row);
        }

        HeightMap { map }
    }

    pub fn get_visible(&self) -> i32 {
        let mut count = 0;
        for row_index in 0..self.map.len() {
            let row = self.map.get(row_index).unwrap();
            for col_index in 0..row.len() {
                let current_tree = row.get(col_index).unwrap();

                if HeightMap::is_visible_in_row(current_tree, &row[..col_index])
                || HeightMap::is_visible_in_row(current_tree, &row[(col_index + 1)..])
                || self.is_visible_in_col(current_tree, &row_index, &col_index){
                    count += 1;
                }
                
            }
        }

        count
    }

    pub fn get_highest_scenic_score(&self) -> usize {
        let mut max_score = 0;


        for row_index in 0..self.map.len() {
            let row = self.map.get(row_index).unwrap();
            for col_index in 0..row.len() {

                if col_index == 0 || col_index == row.len() - 1 || row_index == 0 || row_index == self.map.len() - 1 {
                    continue;
                }

                let current_tree = row.get(col_index).unwrap();

                let count_up = self.trees_visible_up(&row_index, &col_index, current_tree);
                let count_down = self.trees_visible_down(&row_index, &col_index, current_tree);
                let count_left = self.trees_visible_left(row, &col_index, current_tree);
                let count_right = self.trees_visible_right(row, &col_index, current_tree);
                
                let scenic_score = count_down * count_left * count_right * count_up;

                if scenic_score > max_score {
                    max_score = scenic_score;
                }
            }
        }

        max_score
    }

    fn is_visible_in_row(current_tree: &usize, other_row_part: &[usize]) -> bool {
        other_row_part.iter().all(|x| current_tree > x)
    }

    fn is_visible_in_col(&self, current_tree: &usize, row_index: &usize, col_index: &usize) -> bool {
        self.map[..*row_index].iter().all(|row| row.get(*col_index).unwrap() < current_tree)
        || self.map[(*row_index + 1)..].iter().all(|row| row.get(*col_index).unwrap() < current_tree)
    }

    fn trees_visible_up(&self, current_tree_row: &usize, current_tree_col: &usize, current_tree: &usize) -> usize {
        let rows_up: Vec<&Vec<usize>> = self.map[..(*current_tree_row)].iter().rev().collect();
        HeightMap::trees_visible_in_column(&rows_up, current_tree_col, current_tree)
    }

    fn trees_visible_down(&self, current_tree_row: &usize, current_tree_col: &usize, current_tree: &usize) -> usize {
        let rows_down: Vec<&Vec<usize>> = self.map[(*current_tree_row + 1)..].iter().collect();
        HeightMap::trees_visible_in_column(&rows_down, current_tree_col, current_tree)
    }

    fn trees_visible_in_column(rows: &[&Vec<usize>], current_tree_col: &usize, current_tree: &usize) -> usize {
        let a = rows.iter()
            .map(|x| x.get(*current_tree_col).unwrap())
            .take_while(|x| **x < *current_tree);
        cmp::min(a.count() + 1, rows.len())
    }

    fn trees_visible_left(&self, current_row: &[usize], current_tree_col: &usize, current_tree: &usize) -> usize {
        let row_left: Vec<usize> = current_row[..(*current_tree_col)].iter().copied().rev().collect();
        HeightMap::trees_visible_in_row(&row_left, current_tree)
    }

    fn trees_visible_right(&self, current_row: &[usize], current_tree_col: &usize, current_tree: &usize) -> usize {
        let row_right: Vec<usize> = current_row[(*current_tree_col + 1)..].to_vec();
        HeightMap::trees_visible_in_row(&row_right, current_tree)
    }

    fn trees_visible_in_row(row: &[usize], current_tree: &usize) -> usize {
        let a = row.iter()
            .take_while(|x| *x < current_tree);

        cmp::min(a.count() + 1, row.len())
    }

}
pub struct Stacks {
    data: Vec<Stack>,
}

impl Stacks {
    pub fn from_str_lines(str_lines: &[String]) -> Option<Stacks> {
        let mut stacks: Vec<Stack> = Vec::new();
        
        for (i, line) in str_lines.iter().rev().enumerate() {
            if i == 0 {
                let stacks_count = line.split_ascii_whitespace().count();

                for _ in 0..stacks_count {
                    stacks.push(Stack::default());
                }

                continue;
            }

            for stack_index in 0.. stacks.len() {
                let stack_pos = stack_index * 4 + 1;
                let crate_letter = line.chars().nth(stack_pos).unwrap();

                if crate_letter != ' ' {
                    stacks.get_mut(stack_index).unwrap().push(crate_letter);
                }
            }

        }

        Some(Stacks {
            data: stacks
        })
    }

    pub fn move_x_from_a_to_b(&mut self, x: usize, a: usize, b: usize) {
        for _ in 0..x {
            let from_stack = self.data.get_mut(a - 1).unwrap();
            let crate_to_move = from_stack.pop().unwrap();

            let to_stack = self.data.get_mut(b - 1).unwrap();
            to_stack.push(crate_to_move);
        }
    }

    pub fn move_x_from_a_to_b_with_crate_mover_9001(&mut self, x: usize, a: usize, b: usize) {
        let from_stack = self.data.get_mut(a - 1).unwrap();
        let crates_to_move = from_stack.pop_multiple(x).unwrap();

        let to_stack = self.data.get_mut(b - 1).unwrap();
        to_stack.push_multiple(&crates_to_move);
    }

    pub fn get_last_crates(&self) -> Vec<&char> {
        let mut result = Vec::new();
        for stack in &self.data {

            if let Some(x) = stack.peek() {
                result.push(x);
            }
        }

        result
    }
}


#[derive(Default)]
pub struct Stack {
    data: Vec<char>,
}

impl Stack {
    fn push(&mut self, c: char) {
        self.data.push(c)
    }

    fn push_multiple(&mut self, chars: &[char]) {
        self.data.extend_from_slice(chars)
    }

    fn pop(&mut self) -> Option<char> {
        self.data.pop()
    }

    fn pop_multiple(&mut self, count: usize) -> Option<Vec<char>> {
        let start_pos = self.data.len() - count;
        let mut result: Vec<char> = Vec::with_capacity(count);
        result.extend_from_slice(&self.data[start_pos..]);
        self.data.drain(start_pos..);

        Some(result)
    }

    fn peek(&self) -> Option<&char> {
        self.data.last()
    }
}

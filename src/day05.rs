use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::num::ParseIntError;

struct Supplies {
    stacks_map: HashMap<String, VecDeque<String>>,
}

#[derive(Debug, thiserror::Error)]
enum SuppliesError {
    #[error("Malformed input.")]
    ParseFailure,
    #[error("Cannot remove elements from an empty stack.")]
    EmptyStack,
    #[error("Provided stack key does not exist: {0}.")]
    MissingKey(String),
}

impl Supplies {
    fn try_from(puzzle_input: &str) -> Result<Self, SuppliesError> {
        let mut puzzle_lines: Vec<&str> = puzzle_input.split('\n').collect();
        let stack_keys_line = puzzle_lines.remove(puzzle_lines.len() - 1);
        let stack_keys: Vec<&str> = stack_keys_line.split_whitespace().collect();
        let mut stacks_map: HashMap<String, VecDeque<String>> = HashMap::new();

        for line in puzzle_lines {
            let mut chars = line.chars();
            for stack_key in &stack_keys {
                chars.next(); // [
                let supply_crate = chars.next().ok_or(SuppliesError::ParseFailure)?;
                chars.next(); // ]

                stacks_map
                    .entry(String::from(*stack_key))
                    .or_insert(VecDeque::new());
                if supply_crate.is_alphabetic() {
                    // 'unwrap()' is suitable in this case as the above 'or_insert' ensures that the key exists.
                    stacks_map
                        .get_mut(&String::from(*stack_key))
                        .unwrap()
                        .push_front(String::from(supply_crate));
                }
                chars.next();
            }
        }

        Ok(Self { stacks_map })
    }

    fn move_crates_9000(&mut self, n: usize, src: &str, dst: &str) -> Result<(), SuppliesError> {
        let src_stack = self
            .stacks_map
            .get_mut(src)
            .ok_or(SuppliesError::MissingKey(String::from(src)))?;
        let mut src_crates = VecDeque::new();
        for _ in 0..n {
            // `panic` is preferred otherwise the caller could use a supplies instance in an invalid state.
            // A bad state is when some assumption, guarantee, contract, or invariant has been broken.
            // If this code panics, it indicates a bug on the caller's side.
            let src_crate = src_stack.pop_back().expect("non-empty supply stack");
            src_crates.push_back(src_crate);
        }
        let dst_stack = self
            .stacks_map
            .get_mut(dst)
            .ok_or(SuppliesError::MissingKey(String::from(src)))?;
        dst_stack.append(&mut src_crates);
        Ok(())
    }

    fn move_crates_9001(&mut self, n: usize, src: &str, dst: &str) -> Result<(), SuppliesError> {
        let src_stack = self
            .stacks_map
            .get_mut(src)
            .ok_or(SuppliesError::MissingKey(String::from(src)))?;
        let mut src_crates = src_stack.split_off(src_stack.len() - n);
        let dst_stack = self
            .stacks_map
            .get_mut(dst)
            .ok_or(SuppliesError::MissingKey(String::from(src)))?;
        dst_stack.append(&mut src_crates);
        Ok(())
    }

    fn get_stack_tops(&self) -> Result<String, SuppliesError> {
        let mut sorted_keys: Vec<String> = self.stacks_map.keys().cloned().collect();
        sorted_keys.sort();

        let mut tops: Vec<String> = Vec::new();
        for key in sorted_keys {
            let stack = self.stacks_map.get(&key).unwrap();
            let top = stack.back().ok_or(SuppliesError::EmptyStack)?;
            tops.push(top.clone());
        }

        Ok(tops.join(""))
    }
}

fn parse_instruction(instruction: &str) -> Result<(usize, String, String), ParseIntError> {
    let mut words = instruction.split(' ');
    let n = words.nth(1).unwrap().parse::<usize>()?;
    let src = words.nth(1).unwrap();
    let dst = words.nth(1).unwrap();
    Ok((n, String::from(src), String::from(dst)))
}

pub fn part1(file_path: &str) -> String {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let mut puzzle_split = puzzle_input.split("\n\n");
    let mut supplies = Supplies::try_from(puzzle_split.next().unwrap()).unwrap();

    for instruction in puzzle_split.next().unwrap().split('\n') {
        let (n, src, dst) = parse_instruction(instruction).unwrap();
        supplies.move_crates_9000(n, &src, &dst).unwrap();
    }
    supplies.get_stack_tops().unwrap()
}

pub fn part2(file_path: &str) -> String {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let mut puzzle_split = puzzle_input.split("\n\n");
    let mut supplies = Supplies::try_from(puzzle_split.next().unwrap()).unwrap();

    for instruction in puzzle_split.next().unwrap().split('\n') {
        let (n, src, dst) = parse_instruction(instruction).unwrap();
        supplies.move_crates_9001(n, &src, &dst).unwrap();
    }
    supplies.get_stack_tops().unwrap()
}

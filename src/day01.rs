// cargo clippy; rustfmt src/day01.rs

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

// &str is an immutable reference to a string slice.
// Using string slice (&str) as a parameter allows the use of both &String and &str values in the method.
pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let puzzle_input_chunks = puzzle_input.split("\n\n");

    let mut calorie_totals: Vec<u32> = Vec::new();
    for chunk in puzzle_input_chunks {
        // Type annotation is required here because type inference is complex.
        // https://rustc-dev-guide.rust-lang.org/type-inference.html
        let inventory: Vec<u32> = chunk
            .lines()
            .map(|line| {
                // ::<u32> is a type argument that is passed to the type parameter of the generic parse() method.
                line.parse::<u32>().unwrap()
            })
            .collect();
        // Type annotation is *not* required here because the compiler can infer the type from the variable's usage.
        let total_calories = inventory.iter().sum();
        calorie_totals.push(total_calories);
    }
    // u32 values are immutable which means a copy of the derefenced maximum value is returned.
    return *calorie_totals.iter().max().unwrap();
}

pub fn part2(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut highest_sums = [0, 0, 0];
    let mut current_sum = 0;

    for result in reader.lines() {
        let line = result.unwrap();
        if line.is_empty() {
            replace_min(&mut highest_sums, current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<u32>().unwrap();
        }
    }

    // Passing a mutable *reference* to the argument slice is called a mutable borrow.
    replace_min(&mut highest_sums, current_sum);
    return highest_sums.iter().sum::<u32>();
}

// Using a slice reference &[u32] instead of a vector reference &Vec<u32> in the type definition.
// &Vec<u32> as an argument type would require a vector be allocated on the heap prior to calling the function.
// &Vec<u32> will be automatically coerced into a &[u32].
fn replace_min(vec: &mut [u32], value: u32) {
    let min_index = vec
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, _)| i)
        .unwrap();

    if value > vec[min_index] {
        vec[min_index] = value;
    }
}
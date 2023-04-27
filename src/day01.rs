// 0

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

// 1, 2 & 3
pub fn part1(file_path: &str) -> u32 {
    // 4 & 5
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let puzzle_input_chunks = puzzle_input.split("\n\n");

    let mut calorie_totals: Vec<u32> = Vec::new();

    for chunk in puzzle_input_chunks {
        // 6
        let inventory: Vec<u32> = chunk
            .lines()
            .map(|line| {
                // 7 & 8
                line.parse::<u32>().unwrap()
            })
            .collect();
        // 9
        let total_calories = inventory.iter().sum();
        calorie_totals.push(total_calories);
    }

    // 10
    return *calorie_totals.iter().max().unwrap();
}
// 0. cargo clippy; rustfmt src/day01.rs
//
// 1. &str is an immutable reference to a string slice.
// 2. Using string slice (&str) as a parameter allows the use of both &String and &str values in the method.
// 3. See “Implicit Deref Coercions with Functions and Methods” -
// https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
//
// 4. unwrap() panics if there is an error.
// 5. unwrap() is used because we don't expect the calling method of parse_puzzle_input to handle fs errors.
//
// 6. Type annotation is required here because type inference is complex (trust me; not a friendly rabbit hole) -
// https://rustc-dev-guide.rust-lang.org/type-inference.html
//
// 7. ::<u32> is a type argument that is passed to the type parameter of the generic parse() method
// 8. Larger programs with a longer lifespan would benefit from using expect() instead of unwrap()
//
// 9. Type annotation is *not* required here because type inference is complex -
// The compiler can infer the type from the usage: total_calories is inferred to be u32 because
// it is being added to calorie_totals: Vec<u32>
//
// 10. u32 values are immutable which means a copy of the derefenced maximum value is returned.

// 0, 1, 2 & 3
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

    replace_min(&mut highest_sums, current_sum);
    return highest_sums.iter().sum::<u32>();
}

// 4 & 5
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
// 0. Try hyperfine for benchmarking - https://github.com/sharkdp/hyperfine
//
// 1. "Anybody that thinks just read the code and think about it, that's an insane statement." - John Carmack.
// * Always code with a debugger *
//
// 2. In the part 1 solution, we're holding all the puzzle data in memory (twice).
// We're going to construct a lazy solution this time...
//
// 3. cmd + click to go to definition (VSCode).
//
// 4. Using a slice reference &[u32] instead of a vector reference &Vec<u32> in the type definition.
// &Vec<u32> as an argument type would require a vector be allocated on the heap prior to calling the function.
// &Vec<u32> will be automatically coerced into a &[u32] anyway.
//
// 5. We need to pass a mutable *reference* to the argument slice; This is known as a mutable borrow.
// See https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
// 0

use std::fs;

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
// total_calories is inferred to be u32 because it is being added to calorie_totals: Vec<u32>
//
// 10. u32 values are immutable which means a copy of the derefenced maximum value is returned.

pub fn part2(file_path: &str) -> u32 {
    return 0;
}

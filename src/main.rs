// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
mod day07;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Day 01 part 1: {}", day01::part1("resources/day01.txt"));
    // println!("Day 01 part 2: {}", day01::part2("resources/day01.txt"));

    // println!("Day 02 part 1: {}", day02::part1("resources/day02.txt"));
    // println!("Day 02 part 2: {}", day02::part2("resources/day02.txt"));

    // println!("day 03 part 1: {}", day03::part1("resources/day03.txt"));
    // println!("day 03 part 2: {}", day03::part2("resources/day03.txt"));

    // println!("day 04 part 1: {}", day04::part1("resources/day04.txt"));
    // println!("day 04 part 2: {}", day04::part2("resources/day04.txt"));

    // println!("day 05 part 1: {}", day05::part1("resources/day05.txt"));
    // println!("day 05 part 2: {}", day05::part2("resources/day05.txt"));

    // println!("day 06 part 1: {}", day06::part1("resources/day06.txt"));
    // println!("day 06 part 2: {}", day06::part2("resources/day06.txt"));

    println!("day 07 part 1: {}", day07::part1("resources/day07.txt")?);
    Ok(())
}

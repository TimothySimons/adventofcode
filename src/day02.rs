use std::fs;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

const ROCK_VALUE: u32 = 1;
const PAPER_VALUE: u32 = 2;
const SCISSORS_VALUE: u32 = 3;

#[derive(Copy, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn value(&self) -> u32 {
        match self {
            RPS::Rock => ROCK_VALUE,
            RPS::Paper => PAPER_VALUE,
            RPS::Scissors => SCISSORS_VALUE,
        }
    }

    fn stronger(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn equal(&self) -> RPS {
        *self
    }

    fn weaker(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');

    let mut total = 0;
    for line in lines {
        let mut inputs = line.split(' ');
        let (input1, input2) = (inputs.next().unwrap(), inputs.next().unwrap());
        let (p1, p2) = get_player_choices(input1, input2);
        let (_, score) = play_game(p1, p2);
        total += score;
    }
    total
}

pub fn part2(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');

    let mut total = 0;
    for line in lines {
        let mut inputs = line.split(' ');
        let (input1, input2) = (inputs.next().unwrap(), inputs.next().unwrap());
        let (p1, p2) = get_player_choices2(input1, input2);
        let (_, score) = play_game(p1, p2);
        total += score;
    }
    total
}

// An alternative is to implement the 'TryFrom' trait on 'RPS' - 'impl TryFrom<char> for RPS'.
// 'TryFrom' instead of 'From' because this conversion is fallible, allowing for RPS::try_from(some_char)
fn get_player_choices(input1: &str, input2: &str) -> (RPS, RPS) {
    let p1 = match input1 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!("Unexpected puzzle input: {input1}"),
    };
    let p2 = match input2 {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!("Unexpected puzzle input: {input2}"),
    };
    (p1, p2)
}

fn get_player_choices2(input1: &str, input2: &str) -> (RPS, RPS) {
    let p1 = match input1 {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!("Unexpected puzzle input: {input1}"),
    };
    let p2 = match input2 {
        "X" => p1.weaker(),
        "Y" => p1.equal(),
        "Z" => p1.stronger(),
        _ => panic!("Unexpected puzzle input: {input2}"),
    };
    (p1, p2)
}

fn play_game(p1: RPS, p2: RPS) -> (u32, u32) {
    if p1.weaker() == p2 {
        (p1.value() + WIN, p2.value() + LOSE)
    } else if p1.equal() == p2 {
        (p1.value() + DRAW, p2.value() + DRAW)
    } else {
        (p1.value() + LOSE, p2.value() + WIN)
    }
}

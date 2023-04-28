use std::fs;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

const ROCK_VALUE: u32 = 1;
const PAPER_VALUE: u32 = 2;
const SCISSORS_VALUE: u32 = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
enum RQS {
    Rock,
    Paper,
    Scissors,
}

impl RQS {
    fn value(&self) -> u32 {
        match self {
            RQS::Rock => ROCK_VALUE,
            RQS::Paper => PAPER_VALUE,
            RQS::Scissors => SCISSORS_VALUE,
        }
    }

    fn stronger(&self) -> RQS {
        match self {
            RQS::Rock => RQS::Paper,
            RQS::Paper => RQS::Scissors,
            RQS::Scissors => RQS::Rock,
        }
    }

    fn equal(&self) -> RQS {
        *self
    }

    fn weaker(&self) -> RQS {
        match self {
            RQS::Rock => RQS::Scissors,
            RQS::Paper => RQS::Rock,
            RQS::Scissors => RQS::Paper,
        }
    }
}

pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');

    let mut total = 0;
    for line in lines {
        let mut inputs = line.split(' ');
        let input1 = inputs.next().unwrap();
        let input2 = inputs.next().unwrap();
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
        let input1 = inputs.next().unwrap();
        let input2 = inputs.next().unwrap();
        let (p1, p2) = get_player_choices2(input1, input2);
        let (_, score) = play_game(p1, p2);
        total += score;
    }
    total
}

fn get_player_choices(input1: &str, input2: &str) -> (RQS, RQS) {
    let p1 = match input1 {
        "A" => RQS::Rock,
        "B" => RQS::Paper,
        "C" => RQS::Scissors,
        _ => panic!("Unexpected puzzle input: {input1}"),
    };
    let p2 = match input2 {
        "X" => RQS::Rock,
        "Y" => RQS::Paper,
        "Z" => RQS::Scissors,
        _ => panic!("Unexpected puzzle input: {input2}"),
    };
    (p1, p2)
}

fn get_player_choices2(input1: &str, input2: &str) -> (RQS, RQS) {
    let p1 = match input1 {
        "A" => RQS::Rock,
        "B" => RQS::Paper,
        "C" => RQS::Scissors,
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

fn play_game(p1: RQS, p2: RQS) -> (u32, u32) {
    if p1 == p2 {
        return (p1.value() + DRAW, p2.value() + DRAW);
    } else if (p1 == RQS::Rock && p2 == RQS::Scissors)
        || (p1 == RQS::Paper && p2 == RQS::Rock)
        || (p1 == RQS::Scissors && p2 == RQS::Paper)
    {
        return (p1.value() + WIN, p2.value() + LOSE);
    }
    (p1.value() + LOSE, p2.value() + WIN)
}

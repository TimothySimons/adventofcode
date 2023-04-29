// LEARNING TOPIC: Data Modelling

use std::collections::HashMap;
use std::fs;

// 'for' loops (like functions) cannot be used with 'const' or 'static'
//  - 'const' values are inlined to each place they're used at compile time.
//  - 'static' values are not inlined like 'const' but reside at a fixed location in memory (evaluated at compile time).
// 'lazy_static' allows 'static's that require code to be executed at runtime (lazily when they are first accessed).
//  - 'lazy_static' is useful for storing thread-safe global variables and shared constant data.
//  - overkill for this simple program, but we're learning :) and this is a useful crate to be aware of.
//  - https://blog.logrocket.com/rust-lazy-static-pattern
// Alternatively, 'HashMap' could be passed to the functions that need it (seems unidiomatic).
lazy_static::lazy_static! {
    // 'static ref' is not part of the language, it's part of 'lazy_static'.
    static ref PRIORITY_MAP: HashMap<char, u32> = {
        let mut map: HashMap<char, u32> = HashMap::new();
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        for (i, c) in alphabet.chars().enumerate() {
            map.insert(c, (i + 1) as u32);
        }
        for (i, c) in alphabet.to_uppercase().chars().enumerate() {
            map.insert(c, (i + 27) as u32);
        }
        map
    };
}

#[derive(PartialEq)]
struct Item {
    letter: char,
    priority: u32,
}

#[derive(Debug)]
enum ItemError {
    InvalidChar(char),
}

impl Item {
    fn new(letter: char) -> Result<Self, ItemError> {
        if !letter.is_alphabetic() {
            return Err(ItemError::InvalidChar(letter));
        }
        let priority = *PRIORITY_MAP.get(&letter).unwrap();
        Ok(Self { letter, priority })
    }
}

struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    fn try_from(input: &str) -> Result<Self, ItemError> {
        let mut items = Vec::new();
        for char in input.chars() {
            match Item::new(char) {
                Ok(item) => items.push(item),
                Err(e) => return Err(e),
            };
        }
        Ok(Self { items })
    }
}

struct RuckSack {
    compartment1: Compartment,
    compartment2: Compartment,
}

impl RuckSack {
    fn try_from(input: &str) -> Result<Self, ItemError> {
        let (left, right) = input.split_at(input.len() / 2);
        let compartment1 = match Compartment::try_from(left) {
            Ok(compartment) => compartment,
            Err(e) => return Err(e),
        };
        let compartment2 = match Compartment::try_from(right) {
            Ok(compartment) => compartment,
            Err(e) => return Err(e),
        };
        Ok(Self {
            compartment1,
            compartment2,
        })
    }

    fn find_duplicate_priority(&self) -> Option<u32> {
        self.compartment1
            .items
            .iter()
            .find(|&item| self.compartment2.items.contains(item))
            .map(|dup| dup.priority)
    }
}

pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let lines = puzzle_input.split('\n');
    let mut total = 0;
    for line in lines {
        let rucksack = RuckSack::try_from(line).unwrap();
        let priority = rucksack.find_duplicate_priority().unwrap();
        total += priority;
    }
    total
}

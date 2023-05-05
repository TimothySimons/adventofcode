use std::collections::HashMap;
use std::fs;

// 'for' loops (like functions) cannot be used with 'const' or 'static'
//  - 'const' values are inlined to each place they're used at compile time.
//  - 'static' values are not inlined like 'const' but reside at a fixed location in memory 
//    (evaluated at compile time). 
//
// 'lazy_static' allows 'static'-like behaviour on variables that need to be evaluated at runtime
//    (which is done lazily; when the variable is first accessed).
//  - 'lazy_static' is useful for storing thread-safe global variables and shared constant data.
//  - https://blog.logrocket.com/rust-lazy-static-pattern
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

// To prevent instantiation using the default constructor, the 'Item' struct can be wrapped inside
// a 'mod' declaration and the new constructor can be made 'pub' - which makes it accessible by this crate.
// This has the effect of creating an external (to the 'item' module) API for the defined data structure.
// https://doc.rust-lang.org/reference/visibility-and-privacy.html
#[derive(PartialEq)]
struct Item {
    letter: char,
    priority: u32,
}

#[derive(Debug)]
enum ItemError {
    InvalidChar(char),
}

// OOP-style solutions are almost always the incorrect approach.
// - "Object orientated programs are offered as alternatives to correct ones..." ~ Edsger Dijkstra
// - "Object-Oriented Programming is Bad" ~ Brian Will
// - "Object-Oriented Programming — The Trillion Dollar Disaster" ~ Ilya Suzdalnitskiy
// The solution is to only think of data as just data and then write a procedural/functional program.
// (It is still a good idea to encapsulate the data in a data structure.)
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

    fn contains(&self, item: &Item) -> bool {
        self.items.contains(item)
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

    fn contains(&self, item: &Item) -> bool {
        self.compartment1.contains(item) || self.compartment2.contains(item)
    }
}

// "John Carmack on Inlined Code" ~ Jonathan Blow's blog
//  - "If everything is just run out in a 2000 line function, it is obvious which part happens first,
//    and you can be quite sure that the later section will get executed before the frame is rendered."
//  - "Besides awareness of the actual code being executed, inlining functions also has the benefit
//    of not making it possible to call the function from other places", which helps avoid bugs that
//    arise from calling state-edit methods in multiple places (a pitfull that doesn't befall pure functions).
//  - Essentially, if a function is only called from a single place, consider inlining it.
//
// Additionaly, 
//  - Use large comment blocks inside the major function to delimit the (inligned) "minor functions".
//  - Use Rust's {} to enforce scoping rules.
//
// A tangent to say that this function could be inligned...
fn find_group_badge_priority(
    rucksack1: RuckSack,
    rucksack2: RuckSack,
    rucksack3: RuckSack,
) -> Option<u32> {
    for item in rucksack1.compartment1.items {
        if rucksack2.contains(&item) && rucksack3.contains(&item) {
            return Some(item.priority);
        }
    }
    for item in rucksack1.compartment2.items {
        if rucksack2.contains(&item) && rucksack3.contains(&item) {
            return Some(item.priority);
        }
    }
    None
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

pub fn part2(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    let mut puzzle_input_iter = puzzle_input.split('\n').into_iter();
    let mut total = 0;

    while let Some(line) = puzzle_input_iter.next() {
        let rucksack1 = RuckSack::try_from(line).unwrap();
        let rucksack2 = RuckSack::try_from(puzzle_input_iter.next().unwrap()).unwrap();
        let rucksack3 = RuckSack::try_from(puzzle_input_iter.next().unwrap()).unwrap();
        total += find_group_badge_priority(rucksack1, rucksack2, rucksack3).unwrap();
    }
    total
}

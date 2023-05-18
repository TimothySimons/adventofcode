use std::collections::{HashSet, VecDeque};
use std::fs;

fn find_marker_end(puzzle_input: &str, marker_size: usize) -> Option<u32> {
    let mut marker_window = VecDeque::new();
    for (idx, character) in puzzle_input.chars().enumerate() {
        marker_window.push_front(character);
        if marker_window.len() > marker_size {
            marker_window.pop_back().unwrap();
            if is_unique(&marker_window) {
                return Some(idx as u32 + 1);
            }
        }
    }
    None
}

fn is_unique(marker_window: &VecDeque<char>) -> bool {
    let marker_set: HashSet<_> = marker_window.iter().collect();
    marker_window.len() == marker_set.len()
}

pub fn part1(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    find_marker_end(&puzzle_input, 4).unwrap()
}

pub fn part2(file_path: &str) -> u32 {
    let puzzle_input = fs::read_to_string(file_path).unwrap();
    find_marker_end(&puzzle_input, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_packet() {
        let marker_size = 4;
        assert_eq!(
            find_marker_end("bvwbjplbgvbhsrlpgdmjqwftvncz", marker_size).unwrap(),
            5
        );
        assert_eq!(
            find_marker_end("nppdvjthqldpwncqszvftbrmjlhg", marker_size).unwrap(),
            6
        );
        assert_eq!(
            find_marker_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", marker_size).unwrap(),
            10
        );
        assert_eq!(
            find_marker_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", marker_size).unwrap(),
            11
        );
        assert_eq!(find_marker_end("zcfzfwzzq", marker_size), None);
    }

    #[test]
    fn test_find_start_of_message() {
        let marker_size = 14;
        assert_eq!(
            find_marker_end("bvwbjplbgvbhsrlpgdmjqwftvncz", marker_size).unwrap(),
            23
        );
        assert_eq!(
            find_marker_end("nppdvjthqldpwncqszvftbrmjlhg", marker_size).unwrap(),
            23
        );
        assert_eq!(
            find_marker_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", marker_size).unwrap(),
            29
        );
        assert_eq!(
            find_marker_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", marker_size).unwrap(),
            26
        );
        assert_eq!(find_marker_end("zcfzfwzzqfrljwz", marker_size), None);
    }
}

//! Solutions to 2022: Advent of Code day 6
//! By Peter Fornwall

use std::collections::HashSet;

fn find_unique(content: &str, nr: usize) -> usize {
    let s = content.trim().as_bytes();
    for i in 0..s.len() - nr {
        let mut h: HashSet<u8> = HashSet::new();
        if (0..nr).all(|x| h.insert(s[i + x])) {
            return i + nr;
        }
    }
    0
}

fn part1(input: &str) -> usize {
    find_unique(input, 4)
}

fn part2(input: &str) -> usize {
    find_unique(input, 14)
}

fn main() {
    let input = include_str!("../../../inputs/day06_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1804);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2508);
}

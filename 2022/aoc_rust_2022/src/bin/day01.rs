//! Solutions to 2022: Advent of Code day 1
//! By Peter Fornwall

fn parse(content: &str) -> Vec<i64> {
    content
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.trim().parse::<i64>().unwrap()).sum())
        .collect::<Vec<_>>()
}

pub fn part1(elves: &[i64]) -> i64 {
    *elves.iter().max().unwrap()
}

pub fn part2(elves: &[i64]) -> i64 {
    let mut v = elves.to_vec();
    v.sort_unstable();
    v.iter().rev().take(3).sum()
}

fn main() {
    let input = include_str!("../../../inputs/day01_input.txt");
    let elves = parse(input);

    let result_p1 = part1(&elves);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 72240);
    let result_p2 = part2(&elves);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 210957);
}

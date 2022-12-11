//! Solutions to 2022: Advent of Code day 3
//! By Peter Fornwall

use itertools::Itertools;
use std::collections::HashSet;

type Input = Vec<Vec<i32>>;
type Output = i32;

fn parse(content: &str) -> Input {
    content
        .lines()
        .map(|line| {
            let mut v: Vec<i32> = Vec::new();
            for c in line.as_bytes() {
                if c.is_ascii_lowercase() {
                    v.push((c - b'a') as i32 + 1);
                } else {
                    v.push((c - b'A') as i32 + 27);
                }
            }
            v
        })
        .collect()
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        .map(|prios| {
            let (s1, s2) = prios
                .chunks(prios.len() / 2)
                .map(|p| p.iter().collect::<HashSet<_>>())
                .next_tuple()
                .unwrap();
            *s1.intersection(&s2).next().unwrap()
        })
        .sum()
}

fn part2(input: &Input) -> Output {
    input
        .chunks(3)
        .map(|chunk| {
            let (s1, s2, s3) = chunk
                .iter()
                .map(|x| x.iter().copied().collect::<HashSet<_>>())
                .next_tuple()
                .unwrap();
            *(&(&s1 & &s2) & &s3).iter().next().unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../inputs/day03_input.txt");
    let v = parse(input);

    let result_p1 = part1(&v);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 8176);
    let result_p2 = part2(&v);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2689);
}

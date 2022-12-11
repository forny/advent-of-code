//! Solutions to 2022: Advent of Code day 9
//! By Peter Fornwall

use std::collections::HashSet;

fn solve(input: &str, nr: usize) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert((0, 0));
    let mut pos: Vec<(i32, i32)> = vec![(0, 0); nr];
    for line in input.trim().lines() {
        let dir = line.trim().as_bytes()[0];
        let steps = line.trim()[2..].parse::<i32>().unwrap();
        for _ in 0..steps {
            match dir {
                b'U' => pos[0].1 += 1,
                b'D' => pos[0].1 -= 1,
                b'R' => pos[0].0 += 1,
                b'L' => pos[0].0 -= 1,
                _ => panic!(),
            }
            for i in 1..nr {
                let head = pos[i - 1];
                let mut tail = &mut pos[i];

                let is_not_touching = head.0.abs_diff(tail.0) > 1u32 || head.1.abs_diff(tail.1) > 1;
                if is_not_touching {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                    if i == (nr - 1) {
                        seen.insert(*tail);
                    }
                }
            }
        }
    }
    seen.len()
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 10)
}

fn main() {
    let input = include_str!("../../../inputs/day09_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 6209);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2460);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day09_example1.txt");
    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 13);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 1);
}

#[test]
fn test2() {
    let input = include_str!("../../../inputs/day09_example2.txt");
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 36);
}

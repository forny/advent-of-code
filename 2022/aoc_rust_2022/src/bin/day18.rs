//! Solutions to 2022: Advent of Code day 18
//! By Peter Fornwall

use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<Vec<i32>>, [[[i32; 20]; 20]; 20]) {
    let v: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    let mut m = [[[0; 20]; 20]; 20];
    for c in &v {
        m[c[0] as usize][c[1] as usize][c[2] as usize] = 1;
    }
    (v, m)
}

const DIMS: [[i32; 3]; 6] = [
    [1i32, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn count_area(v: &[Vec<i32>], m: &[[[i32; 20]; 20]; 20], no_count: i32) -> usize {
    v.iter()
        .map(|c| {
            DIMS.iter()
                .filter(|d| {
                    let p = [c[0] + d[0], c[1] + d[1], c[2] + d[2]];
                    !(p.iter().all(|x| (0..20).contains(x))
                        && m[p[0] as usize][p[1] as usize][p[2] as usize] != no_count)
                })
                .count()
        })
        .sum::<usize>()
}

fn part1(input: &str) -> usize {
    let (v, m) = parse(input);
    count_area(&v, &m, 0)
}

fn part2(input: &str) -> usize {
    let (v, mut m) = parse(input);

    let mut pos = VecDeque::new();
    assert_eq!(m[0][0][0], 0);
    pos.push_back([0, 0, 0]);
    while let Some(p) = pos.pop_front() {
        if p.iter().all(|x| (0..20).contains(x)) {
            let val = &mut m[p[0] as usize][p[1] as usize][p[2] as usize];
            if *val == 0 {
                *val = 2;
                DIMS.iter()
                    .for_each(|d| pos.push_back([p[0] + d[0], p[1] + d[1], p[2] + d[2]]));
            } else {
                continue;
            }
        }
    }
    count_area(&v, &m, 2)
}

fn main() {
    let input = include_str!("../../../inputs/day18_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 3576);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2066);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day18_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 64);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 58);
}

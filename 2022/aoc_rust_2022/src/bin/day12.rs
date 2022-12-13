//! Solutions to 2022: Advent of Code day 12
//! By Peter Fornwall

use std::collections::{HashMap, VecDeque};

struct Trees {
    pub map: Vec<Vec<i32>>,
    pub start: (i32, i32),
    pub end: (i32, i32),
}

fn parse(input: &str) -> Trees {
    let mut start = (0i32, 0i32);
    let mut end = (0i32, 0i32);
    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x as i32, y as i32);
                        0
                    } else if c == 'E' {
                        end = (x as i32, y as i32);
                        (b'z' - b'a') as i32
                    } else {
                        c as i32 - 'a' as i32
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    Trees { map, start, end }
}

fn search(m: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> Option<i32> {
    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut s = VecDeque::new();
    s.push_back((start, 0));
    while !s.is_empty() {
        let (go_pos, steps) = s.pop_front().unwrap();
        if let Some(seen_steps) = seen.get(&go_pos) {
            if *seen_steps <= steps {
                continue;
            }
        }
        seen.insert(go_pos, steps);
        if go_pos == end {
            continue;
        }
        for diff_pos in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (go_pos.0 as i32 + diff_pos.0, go_pos.1 as i32 + diff_pos.1);
            if new_pos.0 < 0
                || new_pos.0 >= m[0].len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= m.len() as i32
            {
                continue;
            }
            if m[new_pos.1 as usize][new_pos.0 as usize]
                <= m[go_pos.1 as usize][go_pos.0 as usize] + 1
            {
                s.push_back((new_pos, steps + 1));
            }
        }
    }
    seen.get(&end).copied()
}

fn part1(input: &str) -> i32 {
    let trees = parse(input);
    search(&trees.map, trees.start, trees.end).unwrap()
}

fn part2(input: &str) -> i32 {
    let trees = parse(input);
    let m = &trees.map;
    let mut shortest = i32::MAX;
    for y in 0..m.len() {
        for x in 0..m[0].len() {
            if m[y][x] == 0 {
                if let Some(nr_steps) = search(m, (x as i32, y as i32), trees.end) {
                    shortest = shortest.min(nr_steps);
                }
            }
        }
    }
    shortest
}

fn main() {
    let input = include_str!("../../../inputs/day12_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 520);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 508);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day12_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 31);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 29);
}

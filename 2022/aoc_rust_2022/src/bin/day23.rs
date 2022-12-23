//! Solutions to 2022: Advent of Code day 23
//! By Peter Fornwall

use std::collections::{hash_map::Entry, HashMap, HashSet};

fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut s: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                s.insert((x as i32, y as i32));
            }
        }
    }
    s
}

fn game(s: &mut HashSet<(i32, i32)>, max_rounds: Option<usize>) -> usize {
    let dirs = [
        [(0, -1), (-1, -1), (1, -1)], // N
        [(0, 1), (1, 1), (-1, 1)],    // S
        [(-1, 0), (-1, -1), (-1, 1)], // W
        [(1, 0), (1, -1), (1, 1)],    // E
    ];

    let mut round = 0;
    loop {
        let mut proposed: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
        for elf in s.iter() {
            let elf_found = (-1..=1).any(|dx| {
                (-1..=1).any(|dy| (dx, dy) != (0, 0) && s.contains(&(elf.0 + dx, elf.1 + dy)))
            });
            if !elf_found {
                continue;
            }
            for dir_ix in 0..dirs.len() {
                let test_dirs = dirs[(round + dir_ix) % 4];
                let elf_found = test_dirs
                    .iter()
                    .any(|(dx, dy)| s.contains(&(elf.0 + dx, elf.1 + dy)));
                if !elf_found {
                    let new_pos = (elf.0 + test_dirs[0].0, elf.1 + test_dirs[0].1);
                    match proposed.entry(new_pos) {
                        Entry::Occupied(mut e) => {
                            e.insert(None);
                        }
                        Entry::Vacant(e) => {
                            e.insert(Some(*elf));
                        }
                    }
                    break;
                }
            }
        }

        round += 1;
        let mut did_any_move = false;
        for (dest, source) in &proposed {
            if let Some(source) = source {
                s.insert(*dest);
                s.remove(source);
                did_any_move = true;
            }
        }
        if !did_any_move {
            return round;
        }
        if let Some(max_rounds) = max_rounds {
            if round == max_rounds {
                return round;
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut s = parse(input);
    game(&mut s, Some(10));

    let max_x = s.iter().map(|elf| elf.0).max().unwrap();
    let max_y = s.iter().map(|elf| elf.1).max().unwrap();
    let min_x = s.iter().map(|elf| elf.0).min().unwrap();
    let min_y = s.iter().map(|elf| elf.1).min().unwrap();
    let squares = (max_x - min_x + 1) * (max_y - min_y + 1);
    let elf_inside = s.len();

    squares as usize - elf_inside
}

fn part2(input: &str) -> usize {
    let mut s = parse(input);
    game(&mut s, None)
}

fn main() {
    let input = include_str!("../../../inputs/day23_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 3877);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 982);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day23_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 110);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 20);
}

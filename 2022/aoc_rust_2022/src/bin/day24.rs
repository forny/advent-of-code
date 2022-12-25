//! Solutions to 2022: Advent of Code day 24
//! By Peter Fornwall

use std::collections::{HashMap, HashSet, VecDeque};

type Bots = Vec<((i32, i32), (i32, i32))>;

fn parse(input: &str) -> (Vec<&[u8]>, Bots) {
    let mut v = Vec::new();
    let dirs = HashMap::from([
        (b'>', (1, 0)),
        (b'v', (0, 1)),
        (b'<', (-1, 0)),
        (b'^', (0, -1)),
    ]);
    let mut b: Bots = Vec::new();
    for (y, line) in input.trim().lines().enumerate() {
        v.push(line.trim().as_bytes());
        for (x, c) in line.trim().as_bytes().iter().enumerate() {
            if dirs.contains_key(c) {
                b.push(((x as i32, y as i32), dirs[c]));
            }
        }
    }
    (v, b)
}

fn go(v: Vec<&[u8]>, b: Bots, forgot_something: bool) -> usize {
    let mut visit: VecDeque<(i32, i32, usize, bool, bool)> = VecDeque::new();
    let mut seen: HashSet<(i32, i32, usize, bool, bool)> = HashSet::new();
    let mut bots: HashSet<(i32, i32)> = HashSet::new();
    let mut bots_time = 0;
    visit.push_back((1, 0, 0, false, false));
    while let Some(p) = visit.pop_front() {
        let (x, y, t, seen_end, seen_start) = p;
        if seen.contains(&(x, y, t, seen_end, seen_start)) {
            continue;
        }
        seen.insert((x, y, t, seen_end, seen_start));

        if bots_time != t {
            bots_time = t;
            bots.clear();
            for bi in &b {
                let x2 = ((bi.0 .0 - 1) + bi.1 .0 * t as i32).rem_euclid(v[0].len() as i32 - 2) + 1;
                let y2 = ((bi.0 .1 - 1) + bi.1 .1 * t as i32).rem_euclid(v.len() as i32 - 2) + 1;
                bots.insert((x2, y2));
            }
        }

        let t2 = t + 1;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)] {
            let x2 = x + dx;
            let y2 = y + dy;
            let mut seen_start2 = seen_start;
            let mut seen_end2 = seen_end;
            if y2 == (v.len() as i32 - 1) && x2 == (v[0].len() as i32 - 2) {
                if seen_start || !forgot_something {
                    return t;
                } else {
                    seen_end2 = true;
                }
            } else if y2 == 0 && x2 == 1 && seen_end {
                seen_start2 = true;
            }
            if y2 >= 0
                && y2 < v.len() as i32
                && v[y2 as usize][x2 as usize] != b'#'
                && !bots.contains(&(x2, y2))
            {
                visit.push_back((x2, y2, t2, seen_end2, seen_start2));
            }
        }
    }
    0
}

fn part1(input: &str) -> usize {
    let (v, b) = parse(input);
    go(v, b, false)
}

fn part2(input: &str) -> usize {
    let (v, b) = parse(input);
    go(v, b, true)
}

fn main() {
    let input = include_str!("../../../inputs/day24_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 279);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 762);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day24_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 18);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 54);
}

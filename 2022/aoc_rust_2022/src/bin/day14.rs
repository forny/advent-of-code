//! Solutions to 2022: Advent of Code day 14
//! By Peter Fornwall

use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<(i32, i32)>, i32) {
    let mut m: HashSet<(i32, i32)> = HashSet::new();
    let mut max_y = 0;
    for line in input.trim().lines() {
        let mut cur = (-1, -1);
        for pair in line.trim().split(" -> ") {
            let p = pair.split_once(',').unwrap();
            let (px, py) = (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap());
            max_y = max_y.max(py);
            if cur.0 != -1 {
                for y in cur.1.min(py)..=cur.1.max(py) {
                    for x in cur.0.min(px)..=cur.0.max(px) {
                        m.insert((x, y));
                    }
                }
            }
            cur = (px, py);
        }
    }
    (m, max_y)
}

fn sand(m: &mut HashSet<(i32, i32)>, max_y: i32, floor_y: i32) -> i32 {
    let mut units: i32 = 0;
    'outer: loop {
        units += 1;
        let mut p = (500, 0);
        loop {
            let mut blocked = true;
            for delta in [(0, 1), (-1, 1), (1, 1)] {
                let new_p = (p.0 + delta.0, p.1 + delta.1);
                if !m.contains(&new_p) {
                    blocked = false;
                    p = new_p;
                    if p.1 == max_y {
                        // Sand flowing into the abyss
                        units -= 1;
                        break 'outer;
                    }
                    break;
                }
            }
            if blocked || p.1 == floor_y - 1 {
                m.insert(p);
                break;
            }
        }
        if p.1 == 0 {
            // Sand became blocked
            break;
        }
    }
    units
}

fn part1(input: &str) -> i32 {
    let (mut m, max_y) = parse(input);
    sand(&mut m, max_y, i32::MAX)
}

fn part2(input: &str) -> i32 {
    let (mut m, max_y) = parse(input);
    sand(&mut m, i32::MAX, max_y + 2)
}

fn main() {
    let input = include_str!("../../../inputs/day14_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 858);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 26845);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day14_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 24);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 93);
}

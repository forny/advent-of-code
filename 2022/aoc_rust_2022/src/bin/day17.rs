//! Solutions to 2022: Advent of Code day 17
//! By Peter Fornwall

use std::collections::{hash_map::Entry, HashMap};

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|x| if x == '>' { 1 } else { -1 })
        .collect()
}

fn does_shape_fit(m: &Vec<[bool; 7]>, s: &[Vec<bool>], x: i32, y: i32) -> bool {
    if y < 0 || x < 0 || x + s[0].len() as i32 > 7 {
        return false;
    }
    let mut fits = true;
    for (dy, shape_line) in s.iter().enumerate() {
        let y_test = y as usize + dy;
        if y_test >= m.len() {
            break;
        }
        for (dx, shape_point) in shape_line.iter().enumerate() {
            let x_test = x as usize + dx;
            if *shape_point && m[y_test][x_test] {
                fits = false;
                break;
            }
        }
    }
    fits
}

fn simulate(input: &str, sim_rounds: i64) -> i64 {
    let v = parse(input);

    let shapes = [
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut sum: i64 = 0;
    let mut m: Vec<[bool; 7]> = Vec::new();
    let mut jet_ix = 0;
    let mut seen = HashMap::new();
    let mut cycle_found = false;
    let mut round = 0;
    while round < sim_rounds {
        let shape_ix = round % 5;

        const HISTORY: usize = 10;
        if !cycle_found && m.len() > HISTORY {
            let top_lines = m[(m.len() - HISTORY)..m.len()].to_vec();
            let key = (top_lines, jet_ix, shape_ix);
            match seen.entry(key) {
                Entry::Occupied(e) => {
                    cycle_found = true;
                    let (last_round, last_height) = e.get();
                    let cycle = round - last_round;
                    let cycle_height = m.len() as i64 - last_height;
                    let complete_cycles_left = (sim_rounds - round) / cycle;
                    println!(
                        "Cycle found at round: {round}, cycle-round-length: {cycle}, cycle-line-height: {cycle_height}"
                    );
                    sum += complete_cycles_left * cycle_height;
                    round = sim_rounds - (sim_rounds - round) % cycle;
                }
                Entry::Vacant(e) => {
                    e.insert((round, sum));
                }
            };
        }

        let shape = &shapes[shape_ix as usize];
        let mut x = 2;
        let mut y = m.len() as i32 + 3;
        loop {
            // move horizontal
            let test_x_pos = x + v[jet_ix];
            jet_ix = (jet_ix + 1) % v.len();
            if does_shape_fit(&m, shape, test_x_pos, y) {
                x = test_x_pos;
            }
            // move vertical
            let test_y_pos = y - 1;
            if does_shape_fit(&m, shape, x, test_y_pos) {
                y = test_y_pos;
            } else {
                let new_lines = (y + shape.len() as i32 - m.len() as i32).max(0);
                (0..new_lines).for_each(|_| m.push([false; 7]));
                sum += new_lines as i64;

                for (shape_y, shape_line) in shape.iter().enumerate() {
                    for (test_x, shape_point) in shape_line.iter().enumerate() {
                        if *shape_point {
                            m[(y + shape_y as i32) as usize][(x + test_x as i32) as usize] = true;
                        }
                    }
                }
                break;
            }
        }
        round += 1;
    }
    sum
}

fn part1(input: &str) -> i64 {
    simulate(input, 2022)
}

fn part2(input: &str) -> i64 {
    simulate(input, 1_000_000_000_000)
}

fn main() {
    let input = include_str!("../../../inputs/day17_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 3159);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 1566272189352);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day17_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 3068);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 1514285714288);
}

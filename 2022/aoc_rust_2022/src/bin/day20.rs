//! Solutions to 2022: Advent of Code day 20
//! By Peter Fornwall

use std::collections::VecDeque;

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}

fn decrypt(v: &[i32], rounds: i32, dec_key: i64) -> i64 {
    let mut seq: VecDeque<_> = v.iter().map(|x| *x as i64 * dec_key).enumerate().collect();

    for _ in 0..rounds {
        for test_ix in 0..v.len() {
            for i in 0..seq.len() {
                let item = (seq[i].0, seq[i].1);
                if item.0 == test_ix {
                    seq.remove(i);
                    let new_ind = (i as i64 + item.1).rem_euclid(seq.len() as i64);
                    seq.insert(new_ind as usize, item);
                    break;
                }
            }
        }
    }

    let ind = seq.iter().position(|item| item.1 == 0).unwrap();
    (1..=3).map(|x| seq[((x * 1000 + ind) % seq.len())].1).sum()
}

fn part1(input: &str) -> i64 {
    let v = parse(input);
    decrypt(&v, 1, 1)
}

fn part2(input: &str) -> i64 {
    let v = parse(input);
    decrypt(&v, 10, 811589153)
}

fn main() {
    let input = include_str!("../../../inputs/day20_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1591);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 14579387544492);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day20_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 3);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 1623178306);
}

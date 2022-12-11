//! Solutions to 2022: Advent of Code day 4
//! By Peter Fornwall

use itertools::Itertools;

type Input = Vec<(i32, i32, i32, i32)>;
type Output = usize;

fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(['-', ','])
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part1(input: &Input) -> Output {
    input
        .iter()
        // Check if one range fully contains the other
        .filter(|x| (x.2 >= x.0 && x.3 <= x.1) || (x.0 >= x.2 && x.1 <= x.3))
        .count()
}

fn part2(input: &Input) -> Output {
    // Check how many overlap
    input.iter().filter(|x| !(x.2 > x.1 || x.3 < x.0)).count()
}

fn main() {
    let input = include_str!("../../../inputs/day04_input.txt");
    let v = parse(input);

    let result_p1 = part1(&v);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 582);
    let result_p2 = part2(&v);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 893);
}

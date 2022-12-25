//! Solutions to 2022: Advent of Code day 25
//! By Peter Fornwall

fn snafu_to_nr(s: &str) -> i64 {
    let mut base: i64 = 1;
    let mut nr = 0;
    for c in s.chars().rev() {
        let x = "=-012".find(c).unwrap() as i64 - 2;
        nr += base * x;
        base *= 5;
    }
    nr
}

fn part1(input: &str) -> String {
    let mut sum: i64 = input.trim().lines().map(snafu_to_nr).sum();

    let mut s = String::new();
    while sum != 0 {
        let rem = (sum + 2) % 5;
        sum = (sum + 2) / 5;
        s.push("=-012".chars().nth(rem as usize).unwrap());
    }
    let snaf = s.chars().rev().collect::<String>();
    snaf
}

fn main() {
    let input = include_str!("../../../inputs/day25_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, "2=20---01==222=0=0-2");
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day25_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, "2=-1=0");
}

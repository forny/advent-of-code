//! Solutions to 2022: Advent of Code day 2
//! By Peter Fornwall

fn parse(content: &str) -> Vec<(i32, i32)> {
    content
        .lines()
        .map(|line| {
            (
                line.as_bytes()[0] as i32 - 'A' as i32,
                line.as_bytes()[2] as i32 - 'X' as i32,
            )
        })
        .collect()
}

fn score(play1: i32, play2: i32) -> i32 {
    let mut s = (play2 + 1) as i32;
    if play1 == play2 {
        s += 3;
    } else if (play1 + 1) % 3 == play2 {
        s += 6;
    }
    s
}

fn part1(values: &[(i32, i32)]) -> i32 {
    values.iter().map(|(p1, p2)| score(*p1, *p2)).sum()
}

fn part2(values: &[(i32, i32)]) -> i32 {
    values
        .iter()
        .map(|&(p1, result)| {
            let r = result - 1;
            let p2 = if r == 0 { p1 } else { (p1 + r).rem_euclid(3) };
            score(p1, p2)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../../inputs/day02_input.txt");
    let v = parse(input);

    let result_p1 = part1(&v);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 10816);
    let result_p2 = part2(&v);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 11657);
}

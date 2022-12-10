//! Solutions to 2022: Advent of Code day 10
//! By Peter Fornwall

fn get_x(content: &str) -> Vec<i32> {
    content
        .trim()
        .lines()
        .flat_map(|line| {
            if line == "noop" {
                vec![0]
            } else {
                vec![0, line.split(' ').nth(1).unwrap().parse::<i32>().unwrap()]
            }
        })
        .scan(1, |x, x_add| {
            let old = *x;
            *x += x_add;
            Some(old)
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    get_x(input)
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i + 1) as i32 * x)
        .sum()
}

fn part2(input: &str) {
    let s = get_x(input).into_iter().enumerate().map(|(i, x)| {
        if (x - (i % 40) as i32).abs() <= 1 {
            '#'
        } else {
            '.'
        }
    });
    for (i, c) in s.enumerate() {
        print!("{c}");
        if (i + 1) % 40 == 0 {
            println!();
        }
    }
}

fn main() {
    let input = include_str!("../../../inputs/day10_input.txt");
    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 13520);
    part2(input);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day10_example1.txt");
    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 13140);
}

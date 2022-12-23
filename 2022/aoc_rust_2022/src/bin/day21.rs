//! Solutions to 2022: Advent of Code day 21
//! By Peter Fornwall

#[derive(Debug)]
enum RightSide {
    Nr(f64),
    Math(String, char, String),
}

fn parse(input: &str) -> Vec<(String, RightSide)> {
    let mut v = Vec::new();
    for line in input.trim().lines() {
        let line = line.trim();
        let s: Vec<_> = line.split(": ").collect();
        let name = s[0];
        if let Ok(nr) = s[1].trim().parse::<f64>() {
            v.push((name.to_string(), RightSide::Nr(nr)));
        } else {
            let math: Vec<_> = s[1].trim().split(' ').collect();
            let n1 = math[0].to_string();
            let op = math[1].chars().next().unwrap();
            let n2 = math[2].to_string();
            v.push((name.to_string(), RightSide::Math(n1, op, n2)));
        }
    }
    v
}

fn get_name(v: &Vec<(String, RightSide)>, name: &str) -> f64 {
    for i in v {
        if i.0 == name {
            return match &i.1 {
                RightSide::Nr(nr) => *nr,
                RightSide::Math(n1, op, n2) => {
                    let v1 = get_name(v, n1);
                    let v2 = get_name(v, n2);
                    match op {
                        '+' => v1 + v2,
                        '-' => v1 - v2,
                        '*' => v1 * v2,
                        '/' => v1 / v2,
                        _ => {
                            panic!();
                        }
                    }
                }
            };
        }
    }
    panic!();
}

fn solve(v: &Vec<(String, RightSide)>, name: &str, tup: (f64, f64)) -> f64 {
    for i in v {
        if let RightSide::Math(n1, op, n2) = &i.1 {
            let (val, factor) = if n1 == name {
                (get_name(v, n2) as f64, 1.0f64)
            } else if n2 == name {
                assert_ne!(*op, '/');
                (get_name(v, n1) as f64, -1.0f64)
            } else {
                continue;
            };
            if i.0 == "root" {
                return (val - tup.1) / tup.0;
            }

            let new_tup = match op {
                '+' => (tup.0, tup.1 + val),
                '-' => (factor * tup.0, factor * (tup.1 - val)),
                '*' => (val * tup.0, val * tup.1),
                '/' => (tup.0 / val, tup.1 / val),
                _ => panic!(),
            };
            return solve(v, &i.0, new_tup);
        }
    }
    panic!();
}

fn part1(input: &str) -> i64 {
    let v = parse(input);
    get_name(&v, "root").round() as i64
}

fn part2(input: &str) -> i64 {
    let v = parse(input);
    solve(&v, "humn", (1.0, 0.0)).round() as i64
}

fn main() {
    let input = include_str!("../../../inputs/day21_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 72664227897438);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 3916491093817);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day21_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 152);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 301);
}

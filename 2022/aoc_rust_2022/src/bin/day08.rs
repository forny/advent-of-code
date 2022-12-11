//! Solutions to 2022: Advent of Code day 8
//! By Peter Fornwall

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut m: Vec<Vec<i32>> = Vec::new();
    for line in input.trim().lines() {
        m.push(line.as_bytes().iter().map(|x| (x - b'0') as i32).collect());
    }
    m
}

fn trees(input: &Vec<Vec<i32>>, mut x: i32, mut y: i32, dx: i32, dy: i32) -> (i32, bool) {
    let height = input[y as usize][x as usize];
    y += dy;
    x += dx;
    let mut c = 0;
    let mut blocked = false;
    while y >= 0 && y < (input.len() as i32) && x >= 0 && x < (input[0].len() as i32) {
        c += 1;
        if input[y as usize][x as usize] >= height {
            blocked = true;
            break;
        }
        y += dy;
        x += dx;
    }
    (c, !blocked)
}

fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for y in 0..input.len() as i32 {
        for x in 0..input[0].len() as i32 {
            let d = trees(input, x, y, 0, 1).1;
            let u = trees(input, x, y, 0, -1).1;
            let r = trees(input, x, y, 1, 0).1;
            let l = trees(input, x, y, -1, 0).1;
            if d || u || r || l {
                sum += 1;
            }
        }
    }
    sum
}
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut max_c = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let d = trees(input, x as i32, y as i32, 0, 1).0;
            let u = trees(input, x as i32, y as i32, 0, -1).0;
            let r = trees(input, x as i32, y as i32, 1, 0).0;
            let l = trees(input, x as i32, y as i32, -1, 0).0;
            let c = d * u * r * l;
            if c > max_c {
                max_c = c;
            }
        }
    }
    max_c
}

fn main() {
    let input = include_str!("../../../inputs/day08_input.txt");
    let v = parse(input);

    let result_p1 = part1(&v);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1835);
    let result_p2 = part2(&v);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 263670);
}

#[test]
fn test1() {
    let input = "30373
25512
65332
33549
35390";
    let v = parse(input);

    let result_p1 = part1(&v);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 21);
    let result_p2 = part2(&v);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 8);
}

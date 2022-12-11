//! Solutions to 2022: Advent of Code day 7
//! By Peter Fornwall

use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, i32> {
    let mut m: HashMap<String, i32> = HashMap::new();
    let lines: Vec<_> = input.trim().lines().collect();
    let mut stack_dirs = Vec::new();
    let mut line_nr = 0;
    while line_nr < lines.len() {
        let line_parts: Vec<_> = lines[line_nr].split(' ').collect();
        if line_parts[1] == "cd" {
            if line_parts[2] == ".." {
                let dir_path = stack_dirs.join("/");
                let calc_size = *m.entry(dir_path).or_insert(0);
                stack_dirs.pop();
                let dir_path = stack_dirs.join("/");
                m.entry(dir_path)
                    .and_modify(|x| {
                        *x += calc_size;
                    })
                    .or_insert(calc_size);
            } else {
                stack_dirs.push(line_parts[2]);
            }
        } else if line_parts[1] == "ls" {
            for (ls_line_nr, ls_line) in lines.iter().enumerate().skip(line_nr + 1) {
                let ls_line_parts: Vec<_> = ls_line.split(' ').collect();
                if ls_line_parts[0] == "$" {
                    line_nr = ls_line_nr - 1;
                    break;
                } else if ls_line_parts[0] == "dir" {
                    continue;
                }
                let dir_size = ls_line_parts[0].parse::<i32>().unwrap();
                let dir_path = stack_dirs.join("/");
                m.entry(dir_path.clone())
                    .and_modify(|x| {
                        *x += dir_size;
                    })
                    .or_insert(dir_size);
            }
        }
        line_nr += 1;
    }

    while stack_dirs.len() > 1 {
        let dir_path = stack_dirs.join("/");
        let calc_size = *m.entry(dir_path).or_insert(0);
        stack_dirs.pop();
        let dir_path = stack_dirs.join("/");
        m.entry(dir_path)
            .and_modify(|x| {
                *x += calc_size;
            })
            .or_insert(calc_size);
    }

    m
}

fn part1(input: &str) -> i32 {
    let m = parse(input);
    let items: Vec<_> = m.iter().filter(|x| x.1 <= &100_000).collect();
    items.iter().map(|x| x.1).sum()
}

fn part2(input: &str) -> i32 {
    let m = parse(input);
    let mut items: Vec<_> = m.iter().collect();
    items.sort_by_key(|k| k.1);
    let root_size = m["/"];
    let left = 70_000_000 - root_size;
    let to_delete = 30_000_000 - left;
    for item in items {
        if *item.1 >= to_delete {
            return *item.1;
        }
    }
    0
}

fn main() {
    let input = include_str!("../../../inputs/day07_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1182909);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2832508);
}

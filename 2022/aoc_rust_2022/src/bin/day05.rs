//! Solutions to 2022: Advent of Code day 5
//! By Peter Fornwall

use std::iter;

struct Crates {
    pub stacks: Vec<Vec<u8>>,
    pub instructions: Vec<(i32, usize, usize)>,
}

fn parse(input: &str) -> Crates {
    let mut it = input.trim().split("\n\n");
    let mut it_lines = it.next().unwrap().lines().rev();
    let nr_stacks = it_lines.next().unwrap().trim().as_bytes().last().unwrap() - b'0';
    let mut stacks: Vec<Vec<u8>> = iter::repeat_with(Vec::new)
        .take(nr_stacks as usize)
        .collect();
    it_lines.for_each(|line| {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.as_bytes()[i * 4 + 1];
            if c != b' ' {
                stack.push(c);
            }
        }
    });

    let mut instructions = Vec::new();
    for line in it.next().unwrap().trim().lines() {
        let in_line = line.trim().split(' ').collect::<Vec<_>>();
        let count = in_line[1].parse::<i32>().unwrap();
        let index_from = in_line[3].parse::<usize>().unwrap() - 1;
        let index_to = in_line[5].parse::<usize>().unwrap() - 1;
        instructions.push((count, index_from, index_to));
    }
    Crates {
        stacks,
        instructions,
    }
}

fn part1(input: &str) -> String {
    let mut crates = parse(input);
    for (count, index_from, index_to) in crates.instructions {
        for _ in 0..count {
            let c = crates.stacks[index_from].pop().unwrap();
            crates.stacks[index_to].push(c);
        }
    }
    let mut s = String::new();
    for stack in crates.stacks {
        s.push(*stack.last().unwrap() as char);
    }
    s
}

fn part2(input: &str) -> String {
    let mut crates = parse(input);
    for (count, index_from, index_to) in crates.instructions {
        let mut moves: Vec<u8> = Vec::new();
        for _ in 0..count {
            let c = crates.stacks[index_from].pop().unwrap();
            moves.push(c);
        }
        moves.reverse();
        for c in moves {
            crates.stacks[index_to].push(c);
        }
    }
    let mut s = String::new();
    for stack in crates.stacks {
        s.push(*stack.last().unwrap() as char);
    }
    s
}

fn main() {
    let input = include_str!("../../../inputs/day05_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, "QNNTGTPFN");
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, "GGNPJBTTR");
}

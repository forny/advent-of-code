//! Solutions to 2022: Advent of Code day 11
//! By Peter Fornwall

use std::mem;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    pub items: Vec<i64>,
    pub op: Op,
    pub test_div: i64,
    pub test_true_monkey: u8,
    pub test_false_monkey: u8,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for monkey in input.trim().split("\n\n") {
        let lines = monkey.lines().collect::<Vec<_>>();
        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let op = if lines[2].contains('+') {
            Op::Add(lines[2].split("+ ").nth(1).unwrap().parse::<i64>().unwrap())
        } else {
            let mul_2nd = lines[2].split("* ").nth(1).unwrap();
            if mul_2nd == "old" {
                Op::Square
            } else {
                Op::Mul(mul_2nd.parse::<i64>().unwrap())
            }
        };
        let test_div = lines[3]
            .split("by ")
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let test_true_monkey = lines[4].as_bytes().last().unwrap() - b'0';
        let test_false_monkey = lines[5].as_bytes().last().unwrap() - b'0';
        monkeys.push(Monkey {
            items,
            op,
            test_div,
            test_true_monkey,
            test_false_monkey,
        });
    }
    monkeys
}

fn do_monkey_worries<F>(monkeys_input: &[Monkey], nr_rounds: i32, worry_handler: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut monkeys = monkeys_input.to_vec();
    let mut inspects = vec![0; monkeys.len()];
    for _ in 0..nr_rounds {
        for monkey_index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_index].items);
            for mut item in items {
                inspects[monkey_index] += 1;
                match monkeys[monkey_index].op {
                    Op::Add(add) => item += add,
                    Op::Mul(mul) => item *= mul,
                    Op::Square => item *= item,
                }
                item = worry_handler(item);
                let throw_to = if item % monkeys[monkey_index].test_div == 0 {
                    monkeys[monkey_index].test_true_monkey as usize
                } else {
                    monkeys[monkey_index].test_false_monkey as usize
                };
                monkeys[throw_to].items.push(item);
            }
        }
    }

    inspects.sort();
    inspects.reverse();
    inspects[0] * inspects[1]
}

fn part1(monkeys_input: &[Monkey]) -> i64 {
    do_monkey_worries(monkeys_input, 20, |worry| worry / 3)
}

fn part2(monkeys_input: &[Monkey]) -> i64 {
    let factors = monkeys_input.iter().fold(1i64, |acc, m| acc * m.test_div);
    do_monkey_worries(monkeys_input, 10_000, |worry| worry % factors)
}

fn main() {
    let input = include_str!("../../../inputs/day11_input.txt");
    let monkeys = parse(input);
    let result_p1 = part1(&monkeys);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 95472);
    let result_p2 = part2(&monkeys);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 17926061332);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day11_example1.txt");
    let monkeys = parse(input);
    let result_p1 = part1(&monkeys);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 10605);
    let result_p2 = part2(&monkeys);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2713310158);
}

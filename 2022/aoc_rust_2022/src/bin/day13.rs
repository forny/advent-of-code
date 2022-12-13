//! Solutions to 2022: Advent of Code day 13
//! By Peter Fornwall

use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Item {
    Number(usize),
    List(Vec<Item>),
}

fn parse_list(input: &str) -> Item {
    let b = input.as_bytes();

    let mut stack: Vec<Vec<Item>> = Vec::new();
    let mut cur_list: Vec<Item> = Vec::new();
    let mut i: usize = 0;
    while i < b.len() {
        if b[i] == b'[' {
            stack.push(cur_list);
            cur_list = Vec::new();
            i += 1;
        } else if b[i] == b']' {
            stack.last_mut().unwrap().push(Item::List(cur_list));
            cur_list = stack.pop().unwrap();
            i += 1;
        } else if b[i] == b',' {
            i += 1;
        } else {
            let mut nr = 0;
            while b[i].is_ascii_digit() {
                nr *= 10;
                nr += b[i] - b'0';
                i += 1;
            }
            cur_list.push(Item::Number(nr as usize));
        }
    }
    Item::List(cur_list)
}

fn parse(input: &str) -> Vec<(Item, Item)> {
    input
        .trim()
        .split("\n\n")
        .map(|s| {
            let p = s.split_once('\n').unwrap();
            (parse_list(p.0), parse_list(p.1))
        })
        .collect()
}

fn compare(item1: &Item, item2: &Item) -> Ordering {
    if let (Item::Number(nr1), Item::Number(nr2)) = (item1, item2) {
        return nr1.cmp(nr2);
    }
    if matches!(item1, Item::Number(_)) {
        let new_list = Item::List(vec![item1.clone()]);
        return compare(&new_list, item2);
    }
    if matches!(item2, Item::Number(_)) {
        let new_list = Item::List(vec![item2.clone()]);
        return compare(item1, &new_list);
    }
    if let (Item::List(list1), Item::List(list2)) = (item1, item2) {
        for (i1, i2) in list1.iter().zip(list2.iter()) {
            let c = compare(i1, i2);
            if c != Ordering::Equal {
                return c;
            }
        }
        return list1.len().cmp(&list2.len());
    }
    panic!();
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    let m = parse(input);
    for (ind, (i1, i2)) in m.iter().enumerate() {
        if compare(i1, i2) != Ordering::Greater {
            sum += ind as i32 + 1;
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let p = parse(input);
    let mut items = Vec::new();
    p.iter().for_each(|(i1, i2)| {
        items.push(i1.clone());
        items.push(i2.clone());
    });
    let key1 = parse_list("[[2]]");
    let key2 = parse_list("[[6]]");
    items.push(key1.clone());
    items.push(key2.clone());

    items.sort_by(compare);
    let mut mul = 1;
    for (ind, item) in items.iter().enumerate() {
        if *item == key1 || *item == key2 {
            mul *= ind + 1;
        }
    }
    mul
}

fn main() {
    let input = include_str!("../../../inputs/day13_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 6656);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 19716);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day13_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 13);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 140);
}

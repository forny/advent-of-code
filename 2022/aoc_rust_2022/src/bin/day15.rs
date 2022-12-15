//! Solutions to 2022: Advent of Code day 15
//! By Peter Fornwall

use itertools::Itertools;
type Point = (i32, i32);
type Scans = Vec<(Point, Point)>;

fn parse(input: &str) -> Scans {
    let mut v = Vec::new();
    for line in input.trim().lines() {
        let splits: Vec<_> = line.trim().split(['=', ',', ':']).collect();
        let x1 = splits[1].parse::<i32>().unwrap();
        let y1 = splits[3].parse::<i32>().unwrap();
        let x2 = splits[5].parse::<i32>().unwrap();
        let y2 = splits[7].parse::<i32>().unwrap();
        v.push(((x1, y1), (x2, y2)));
    }
    v
}

fn make_ranges(scans: &Scans, row: i32) -> Vec<(i32, i32)> {
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    for (sensor, beacon) in scans {
        let dist = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;
        let start_y = (sensor.1 - dist as i32).max(0);
        let end_y = sensor.1 + dist as i32;
        if row >= start_y && row <= end_y {
            let dist_y = sensor.1.abs_diff(row) as i32;
            let start_x = sensor.0 - dist + dist_y;
            let end_x = sensor.0 + dist - dist_y;
            ranges.push((start_x, 1));
            ranges.push((end_x + 1, -1));
        }
    }
    ranges.sort_unstable_by_key(|x| x.0);
    ranges
}

fn part1(input: &str, wanted_y: i32) -> i32 {
    let scans = parse(input);
    let ranges = make_ranges(&scans, wanted_y);
    let mut x = 0;
    let mut overlaps = 0;
    let mut sum = 0;
    for event in &ranges {
        if event.0 > x && overlaps != 0 {
            let beacons_in_range = scans
                .iter()
                .map(|(_, beacon)| beacon)
                .unique()
                .filter(|beacon| beacon.1 == wanted_y && beacon.0 >= x && beacon.0 < event.0)
                .count();
            sum += event.0 - x - beacons_in_range as i32;
        }
        x = event.0;
        overlaps += event.1;
    }
    sum
}

fn part2(input: &str, max_row: i32) -> i64 {
    let scans = parse(input);
    for row in 0..=max_row {
        let ranges = make_ranges(&scans, row);
        let mut x = 0;
        let mut overlaps = 0;
        for range_event in &ranges {
            if range_event.0 > x && overlaps == 0 {
                return x as i64 * 4_000_000 + row as i64;
            }
            x = range_event.0;
            overlaps += range_event.1;
        }
    }
    panic!("No solution found!");
}

fn main() {
    let input = include_str!("../../../inputs/day15_input.txt");

    let result_p1 = part1(input, 2_000_000);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 5112034);
    let result_p2 = part2(input, 4_000_000);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 13172087230812);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day15_example1.txt");

    let result_p1 = part1(input, 10);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 26);
    let result_p2 = part2(input, 20);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 56000011);
}

//! Solutions to 2022: Advent of Code day 19
//! By Peter Fornwall

use std::{collections::HashMap, iter};

fn parse(input: &str) -> Vec<Vec<[i32; 4]>> {
    let mut bps = Vec::new();
    for line in input.trim().lines() {
        let mut bp = Vec::new();
        let line = line.trim();
        let costs = line.split("costs ").collect::<Vec<_>>();
        for (ix, c) in costs.iter().skip(1).enumerate() {
            let mut it = c.split(' ');
            let ore = it.next().unwrap().parse::<i32>().unwrap();
            let mut clay = 0;
            let mut obs = 0;
            if ix == 2 {
                clay = it.nth(2).unwrap().parse::<i32>().unwrap();
            } else if ix == 3 {
                obs = it.nth(2).unwrap().parse::<i32>().unwrap();
            }
            bp.push([ore, clay, obs, 0]);
        }
        bps.push(bp);
    }
    bps
}

fn search(
    bp: &Vec<[i32; 4]>,
    max_res: &[i32],
    geodes: &mut HashMap<(i32, [i32; 4], [i32; 4]), i32>,
    time_left: i32,
    robots: [i32; 4],
    res: [i32; 4],
) -> i32 {
    if time_left == 0 {
        return res[3];
    }
    if let Some(g) = geodes.get(&(time_left, robots, res)) {
        return *g;
    }
    let mut ret = -1;
    let can_construct_geode = res[0] >= bp[3][0] && res[1] >= bp[3][1] && res[2] >= bp[3][2];
    for (bot_ix, bot) in bp.iter().chain(iter::once(&[0, 0, 0, 0])).enumerate() {
        if res.iter().zip(bot.iter()).all(|(res, b)| *res >= *b) {
            if bot_ix != 4 {
                // Don't need more bots then max production cost (except for geode of coourse)
                if bot_ix != 3 && robots[bot_ix] >= max_res[bot_ix] {
                    continue;
                }
            }
            // Always construct geode robot if possible
            if bot_ix != 3 && can_construct_geode {
                continue;
            }
            let mut new_res = res;
            new_res
                .iter_mut()
                .zip(robots.iter())
                .zip(bot.iter())
                .for_each(|((res, r), bp)| *res = *res + *r - *bp);

            // Don't count more ore/clay/obsidian then double what is needed to construct one bot
            for (ix, r) in new_res.iter_mut().enumerate().take(3) {
                *r = (*r).min(2 * max_res[ix]);
            }
            let mut new_robots = robots;
            if bot_ix < 4 {
                new_robots[bot_ix] += 1
            }
            let s = search(bp, max_res, geodes, time_left - 1, new_robots, new_res);
            ret = ret.max(s);
        }
    }
    geodes.insert((time_left, robots, res), ret);
    ret
}

fn search_helper(bp: &Vec<[i32; 4]>, max_time: i32) -> i32 {
    let mut geodes: HashMap<(i32, [i32; 4], [i32; 4]), i32> = HashMap::new();
    let max_res: Vec<_> = (0..4)
        .map(|res_ix| bp.iter().map(|bot| bot[res_ix]).max().unwrap())
        .collect();
    search(bp, &max_res, &mut geodes, max_time, [1, 0, 0, 0], [0; 4])
}

fn part1(input: &str) -> i32 {
    let blueprints = parse(input);
    let mut sum = 0;
    for (ix, bp) in blueprints.iter().enumerate() {
        sum += (ix + 1) as i32 * search_helper(bp, 24);
    }
    sum
}

fn part2(input: &str) -> i32 {
    let blueprints = parse(input);
    let mut product = 1;
    for bp in blueprints.iter().take(3) {
        product *= search_helper(bp, 32);
    }
    product
}

fn main() {
    let input = include_str!("../../../inputs/day19_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1725);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 15510);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day19_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 33);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 56 * 62);
}

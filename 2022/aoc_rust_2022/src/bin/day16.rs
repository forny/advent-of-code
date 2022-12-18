//! Solutions to 2022: Advent of Code day 16
//! By Peter Fornwall

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Valve {
    pub flow: i32,
    pub tunnels: Vec<(usize, usize)>,
}

fn parse(input: &str) -> (Vec<Valve>, usize) {
    let mut m: HashMap<String, usize> = HashMap::new();
    let mut v: Vec<Valve> = Vec::new();

    for line in input.trim().lines() {
        let valve = &line[6..8];
        m.insert(valve.to_string(), m.len());
    }
    let mut conv = HashMap::new();
    let mut conv_ind = 0;
    for (i, line) in input.trim().lines().enumerate() {
        let s = line.trim();
        let valve = &line[6..8];
        let flow_str = s.split(['=', ';']).nth(1).unwrap();
        let flow = flow_str.parse::<i32>().unwrap();
        let tunnels: Vec<_> = s
            .split(", ")
            .map(|s| (m[&s[s.len() - 2..].to_string()], 0))
            .collect();

        v.push(Valve { flow, tunnels });
        if flow > 0 || valve == "AA" {
            conv.insert(i, conv_ind);
            conv_ind += 1;
        }
    }

    let mut v2: Vec<Valve> = Vec::new();
    let start_ind = m["AA"];
    for (ix, valve) in v.iter().enumerate() {
        if valve.flow == 0 && ix != start_ind {
            continue;
        }
        // Eliminate flow=0 valves
        let mut dist: HashMap<usize, i32> = HashMap::new();
        let mut s = VecDeque::new();
        s.push_back((ix, 0));
        while let Some((exp_ix, d)) = s.pop_front() {
            if dist.contains_key(&exp_ix) && dist[&exp_ix] <= d {
                continue;
            }
            dist.insert(exp_ix, d);
            for (tunnel, _) in &v[exp_ix].tunnels {
                s.push_back((*tunnel, d + 1));
            }
        }

        let mut tunnels = Vec::new();
        for (ind, dist) in dist {
            if ix != ind && (v[ind].flow > 0 || ind == start_ind) {
                tunnels.push((conv[&ind], dist as usize));
            }
        }
        v2.push(Valve {
            flow: valve.flow,
            tunnels,
        });
    }

    (v2, conv[&start_ind])
}

fn search(
    valves: &Vec<Valve>,
    valve_idx: usize,
    open_valves: u64,
    time_left: i32,
    seen: &mut HashMap<(usize, u64, i32, bool), i32>,
    do_second: bool,
    start_idx: usize,
) -> i32 {
    if time_left <= 0 {
        if do_second {
            return search(valves, start_idx, open_valves, 26, seen, false, start_idx);
        } else {
            return 0;
        }
    }
    if let Some(seen_sum) = seen.get(&(valve_idx, open_valves, time_left, do_second)) {
        return *seen_sum;
    }

    let mut ret = 0;
    if (open_valves & (1u64 << valve_idx)) == 0 && valves[valve_idx].flow > 0 {
        // Open valve
        ret = (valves[valve_idx].flow * (time_left - 1)
            + search(
                valves,
                valve_idx,
                open_valves | (1u64 << valve_idx),
                time_left - 1,
                seen,
                do_second,
                start_idx,
            ))
        .max(ret);
    }
    for (go_valve, go_dist) in &valves[valve_idx].tunnels {
        ret = search(
            valves,
            *go_valve,
            open_valves,
            time_left - *go_dist as i32,
            seen,
            do_second,
            start_idx,
        )
        .max(ret);
    }

    seen.insert((valve_idx, open_valves, time_left, do_second), ret);
    ret
}

fn part1(input: &str) -> i32 {
    let (valves, start_idx) = parse(input);
    let mut seen: HashMap<(usize, u64, i32, bool), i32> = HashMap::new();
    search(&valves, start_idx, 0u64, 30, &mut seen, false, start_idx)
}

fn part2(input: &str) -> i32 {
    let (valves, start_idx) = parse(input);
    let mut seen: HashMap<(usize, u64, i32, bool), i32> = HashMap::new();
    search(&valves, start_idx, 0u64, 26, &mut seen, true, start_idx)
}

fn main() {
    let input = include_str!("../../../inputs/day16_input.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1701);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 2455);
}

#[test]
fn test1() {
    let input = include_str!("../../../inputs/day16_example1.txt");

    let result_p1 = part1(input);
    println!("Part1: {}", result_p1);
    assert_eq!(result_p1, 1651);
    let result_p2 = part2(input);
    println!("Part2: {}", result_p2);
    assert_eq!(result_p2, 1707);
}

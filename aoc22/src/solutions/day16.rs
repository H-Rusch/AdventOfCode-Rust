use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    leads_to: Vec<String>,
}

pub fn part1(input: &str) -> u32 {
    let valves = parse(input);

    let navigation_map: HashMap<&str, HashMap<&str, u32>> = valves
        .keys()
        .map(|&name| (name, build_navigation_map(name, &valves)))
        .collect();
    let opened: HashSet<&str> = navigation_map
        .keys()
        .filter(|&key| valves.get(key).unwrap().flow_rate == 0)
        .copied()
        .collect();
    let mut cache = HashMap::new();

    dfs(
        "AA",
        1,
        0,
        0,
        30,
        &navigation_map,
        &valves,
        &opened,
        &mut cache,
    )
}

pub fn part2(_input: &str) -> usize {
    0
}

#[allow(clippy::too_many_arguments)]
fn dfs<'a>(
    start: &'a str,
    time: u32,
    flow_rate: u32,
    released: u32,
    limit: u32,
    navigation_map: &'a HashMap<&str, HashMap<&str, u32>>,
    valves: &HashMap<&str, Valve>,
    opened: &HashSet<&str>,
    cache: &mut HashMap<(&'a str, u32, u32, u32), u32>,
) -> u32 {
    // early return if time exceeded
    if time > limit {
        return released - flow_rate * time.abs_diff(limit + 1);
    }

    let state = (start, time, flow_rate, released);
    if let Some(value) = cache.get(&state) {
        return *value;
    }

    // mark current valve as opened
    let mut opened = opened.clone();
    opened.insert(start);

    // update flow rate
    let new_flow_rate = flow_rate + valves.get(start).unwrap().flow_rate;

    // spend 1 minute opening the valve
    let released = released + new_flow_rate;
    let time = time + 1;

    // all valves have been opened -> wait until the time runs out
    if opened.len() == valves.len() {
        return released + new_flow_rate * time.abs_diff(limit + 1);
    }

    let max_release = navigation_map
        .get(start)
        .unwrap()
        .iter()
        .filter(|(name, _)| !opened.contains(**name))
        .map(|(name, cost)| {
            let next_time = time + cost;
            let next_released = released + new_flow_rate * cost;

            dfs(name, next_time, new_flow_rate, next_released, limit, navigation_map, valves, &opened, cache)
        })
        .max()
        .unwrap();

    cache.insert(state, max_release);

    max_release
}

/// Build the map of other vales and costs to reach those starting from a start point. Valves with a flow rate of 0 are not included, because they don't add useful information.
fn build_navigation_map<'a>(start: &'a str, valves: &'a HashMap<&'a str, Valve>) -> HashMap<&'a str, u32> {
    let mut map = HashMap::new();
    let mut expanded = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();

    while let Some((name, cost)) = expanded.pop_front() {
        if visited.contains(name) {
            continue;
        }
        visited.insert(name);

        if valves.get(name).unwrap().flow_rate != 0 && name != start {
            map.insert(name, cost);
        }

        for connected in valves.get(name).unwrap().leads_to.iter() {
            expanded.push_back((connected, cost + 1));
        }
    }

    map
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    let mut valves: HashMap<&str, Valve> = HashMap::new();

    for line in input.lines() {
        let (valve_part, tunnel_part) = line.split_once("; ").unwrap();

        let name = &valve_part[6..=7];
        let flow_rate: u32 = valve_part[8..]
            .strip_prefix(" has flow rate=")
            .unwrap()
            .parse()
            .unwrap();
        let leads_to: Vec<String> = tunnel_part
            .split_whitespace()
            .skip(4)
            .collect::<String>()
            .split(',')
            .map(|name| name.to_string())
            .collect();

        valves.insert(name,Valve { flow_rate, leads_to });
    }

    valves
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day16.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(1651, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(0, part2(INPUT));
    }
}

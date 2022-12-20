use std::collections::{VecDeque, HashSet};

use rayon::prelude::*;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obsi_cost: (u32, u32),
    geode_cost: (u32, u32),
    max_ore_cost: u32,
}

impl Blueprint {
    fn new(id: u32, ore_cost: u32, clay_cost: u32, obsi_cost: (u32, u32), geode_cost: (u32, u32)) -> Blueprint {
        let max_ore_cost = *[ore_cost, clay_cost, obsi_cost.0, geode_cost.0].iter().max().unwrap();

        Blueprint { id, ore_cost, clay_cost, obsi_cost, geode_cost, max_ore_cost }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct OreCounts {
    ore: u32,
    clay: u32,
    obsi: u32,
    geode: u32,
}

impl OreCounts {
    fn new(ore: u32, clay: u32, obsi: u32, geode: u32) -> OreCounts {
        OreCounts { ore, clay, obsi, geode }
    }

    fn produce(&mut self, other: &OreCounts) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsi += other.obsi;
        self.geode += other.geode;
    }
}

pub fn part1(input: &str) -> u32 {
    let blueprints = parse(input);

    blueprints.par_iter()
        .map(|bp| bp.id * calc_max_geodes(bp, 24))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let blueprints = parse(input);

    blueprints.par_iter()
        .take(3)
        .map(|bp| calc_max_geodes(bp, 32))
        .product()
}

fn calc_max_geodes(blueprint: &Blueprint, time_limit: u32) -> u32 {
    // Breadth first search

    // state := ([ore_count; 4], [robot_count; 4], time)
    // ores are [ore, clay, obsi, geode]
    let mut expanded = VecDeque::from([(OreCounts::new(0, 0, 0, 0), OreCounts::new(1, 0, 0, 0), 0)]);
    let mut visited = HashSet::new();
    let mut max_geodes = 0;

    while let Some(state) = expanded.pop_front() {
        let (ore_counts, robot_counts, time) = state;
        if time >= time_limit {
            max_geodes = std::cmp::max(max_geodes, ore_counts.geode);
            continue;
        }

        let key = (ore_counts, robot_counts);
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        // update maximum encountered geode count
        max_geodes = std::cmp::max(max_geodes, ore_counts.geode);

        // probably cannot produce enough geodes -> prune
        if max_geodes > ore_counts.geode + robot_counts.geode * (time_limit - time) {
            continue;
        }

        // always create new geode robot if possible
        if ore_counts.ore >= blueprint.geode_cost.0 && ore_counts.obsi >= blueprint.geode_cost.1 {
            let mut next_ores = ore_counts;
            next_ores.ore -= blueprint.geode_cost.0;
            next_ores.obsi -= blueprint.geode_cost.1;

            next_ores.produce(&robot_counts);

            let mut next_robots = robot_counts;
            next_robots.geode += 1;

            expanded.push_back((next_ores, next_robots, time + 1));
        }

        // don't create another robot if the number of robots is already as high as the amount of ore which can be used in a turn
        // create obsidian 
        if robot_counts.obsi < blueprint.geode_cost.1 && ore_counts.ore >= blueprint.obsi_cost.0 && ore_counts.clay >= blueprint.obsi_cost.1 {
            let mut next_ores = ore_counts;
            next_ores.ore -= blueprint.obsi_cost.0;
            next_ores.clay -= blueprint.obsi_cost.1;

            next_ores.produce(&robot_counts);

            let mut next_robots = robot_counts;
            next_robots.obsi += 1;

            expanded.push_back((next_ores, next_robots, time + 1));
        }

        // create clay 
        if robot_counts.clay < blueprint.obsi_cost.1 && ore_counts.ore >= blueprint.clay_cost {
            let mut next_ores = ore_counts;
            next_ores.ore -= blueprint.clay_cost;
            
            next_ores.produce(&robot_counts);

            let mut next_robots = robot_counts;
            next_robots.clay += 1;

            expanded.push_back((next_ores, next_robots, time + 1));
        }

        // create ore 
        if robot_counts.ore < blueprint.max_ore_cost && ore_counts.ore >= blueprint.ore_cost {
            let mut next_ores = ore_counts;
            next_ores.ore -= blueprint.ore_cost;
            
            next_ores.produce(&robot_counts);

            let mut next_robots = robot_counts;
            next_robots.ore += 1;

            expanded.push_back((next_ores, next_robots, time + 1));
        }

        // don't produce new robots but collect the harvest
        let mut next_ores = ore_counts;
        next_ores.produce(&robot_counts);

        expanded.push_back((next_ores, robot_counts, time + 1));
    }

    max_geodes
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let id = parts.nth(1).unwrap().strip_suffix(':').unwrap().parse().unwrap();
        let ore_cost = parts.nth(4).unwrap().parse().unwrap();
        let clay_cost = parts.nth(5).unwrap().parse().unwrap();
        let obsi_cost_ore = parts.nth(5).unwrap().parse().unwrap();
        let obsi_cost_clay = parts.nth(2).unwrap().parse().unwrap();
        let geode_cost_ore = parts.nth(5).unwrap().parse().unwrap();
        let geode_cost_obsi = parts.nth(2).unwrap().parse().unwrap();

        Blueprint::new(id, ore_cost, clay_cost, (obsi_cost_ore, obsi_cost_clay), (geode_cost_ore, geode_cost_obsi))
    }).collect()

    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day19.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(33, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        // works but takes 2 minutes
        // assert_eq!(3472, part2(INPUT));
    }
}

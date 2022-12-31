use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

lazy_static! {
    static ref REGGI: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

// obsidian, clay, ore
type Resources = (usize, usize, usize);

struct State {
    time_remaining: usize,
    collected_geodes: usize,
    collected_resources: Resources,
    robots: Resources,
}

fn resource_add(a: Resources, b: Resources) -> Resources {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn resource_sub(a: Resources, b: Resources) -> Option<Resources> {
    if a.0 < b.0 || a.1 < b.1 || a.2 < b.2 {
        None
    } else {
        Some((a.0 - b.0, a.1 - b.1, a.2 - b.2))
    }
}

fn resource_mul(a: Resources, f: usize) -> Resources {
    (a.0 * f, a.1 * f, a.2 * f)
}

struct Factory {
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
    max_ore: usize,
}

impl Factory {
    fn new(
        ore_ore: usize,
        clay_ore: usize,
        obsidian_ore: usize,
        obsidian_clay: usize,
        geode_ore: usize,
        geode_obsidian: usize,
    ) -> Self {
        let max_ore = ore_ore.max(clay_ore).max(obsidian_ore).max(geode_ore);
        Self {
            ore_cost: (0, 0, ore_ore),
            clay_cost: (0, 0, clay_ore),
            obsidian_cost: (0, obsidian_clay, obsidian_ore),
            geode_cost: (geode_obsidian, 0, geode_ore),
            max_ore: max_ore,
        }
    }
}

impl State {
    fn new(time: usize) -> Self {
        Self {
            time_remaining: time,
            collected_geodes: 0,
            collected_resources: (0, 0, 0),
            robots: (0, 0, 1),
        }
    }
    fn best_case(&self, f: &Factory) -> usize {
        // assume that only each turn can spawn either a geode or obsidan robot
        // infinite clay and ore
        let tup = (0..self.time_remaining).rev().fold(  // might no need to check 0
            (
                self.collected_geodes,
                self.collected_resources.0,  // obsidian
                self.robots.0  // obsidian robots
            ),
            |(geodes, obsidian, obsidian_robots), time| {
                if obsidian >= f.geode_cost.0 {
                    (  // can make geode robot
                        geodes + time,
                        obsidian - f.geode_cost.0 + obsidian_robots,
                        obsidian_robots
                    )
                } else {
                    (
                        geodes,
                        obsidian + obsidian_robots,
                        obsidian_robots + 1
                    )
                }
            }
        );
        return tup.0;
    }

    fn wait_and_create(&self, cost: &Resources, produces: &Resources) -> Option<Self> {
        for (remaining, elapsed) in (1..self.time_remaining).rev().zip(0..) {
            let current_resources = resource_add(self.collected_resources, resource_mul(self.robots, elapsed));
            if let Some(remaining_resources) = resource_sub(current_resources, *cost) {
                return Some(
                    Self {
                        time_remaining: remaining,
                        collected_geodes: self.collected_geodes,
                        collected_resources: resource_add(remaining_resources, self.robots),
                        robots: resource_add(self.robots, *produces)
                    }
                )
            }
        }
        return None;
    }

    // return something that implements the iterator trait over State
    fn mutations(&self, f: &Factory) -> impl Iterator<Item=Self> {
        let mut retval: Vec<Option<Self>> = Vec::new();
        if self.robots.2 < f.max_ore {  // should create ore robot, else have enough
            retval.push(self.wait_and_create(&f.ore_cost, &(0, 0, 1)));
        }
        if self.robots.1 < f.obsidian_cost.1 {  // should create clay robot
            retval.push(self.wait_and_create(&f.clay_cost, &(0, 1, 0)));
        }
        if self.robots.0 < f.geode_cost.0 {  // should create obsidian robot
            retval.push(self.wait_and_create(&f.obsidian_cost, &(1, 0, 0)));
        }
        for (remaining, elapsed) in (1..self.time_remaining).rev().zip(0..) {
            let current_resources = resource_add(self.collected_resources, resource_mul(self.robots, elapsed));
            if let Some(remaining_resources) = resource_sub(current_resources, f.geode_cost) {
                retval.push(Some(State{
                    time_remaining: remaining,
                    collected_geodes: self.collected_geodes + remaining,
                    collected_resources: resource_add(remaining_resources, self.robots),
                    robots: self.robots
                }));
                break;
            }
        }
        // always create geode robot
        return retval.into_iter().flatten();
    }
}

fn iterate(state: State, f: &Factory) -> usize {
    // current max is state's collected geodes
    let mut geodes = state.collected_geodes;
    for next_state in state.mutations(f) {
        if state.best_case(f) > geodes {  // do not waste time on states which can't even in the best case reach current max
            geodes = geodes.max(iterate(next_state, f));
        }
    }
    return geodes;
}

fn solve(state: State, f: &Factory) -> usize {
    let solution = iterate(state, f);
    return solution;
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut solution = 0;

    for line in reader.lines() {
        let ll = line?;
        if let Some(ccs) = REGGI.captures(&ll) {
            let bp_id = ccs.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let ore_ore_cost = ccs.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let clay_ore_cost = ccs.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let obsidian_ore_cost = ccs.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let obsidian_clay_cost = ccs.get(5).unwrap().as_str().parse::<usize>().unwrap();
            let geode_ore_cost = ccs.get(6).unwrap().as_str().parse::<usize>().unwrap();
            let geode_obsidian_cost = ccs.get(7).unwrap().as_str().parse::<usize>().unwrap();

            let f = Factory::new(
                ore_ore_cost,
                clay_ore_cost,
                obsidian_ore_cost,
                obsidian_clay_cost,
                geode_ore_cost,
                geode_obsidian_cost,
            );
            solution += bp_id * solve(State::new(24), &f);
        } else {
            panic!("Incorrect input {}", ll);
        }
    }

    println!("{}", solution);

    return Ok(());
}

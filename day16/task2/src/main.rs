/*
 * Thanks to janiorca
 * https://github.com/janiorca/advent-of-code-2022/blob/main/src/bin/aoc16.rs
 * TODO: add multithreading, remove state copies on iterations
 */
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct ValveRoom {
    flow: i32,
    all_distances: HashMap<String, i32>, // distance from this node to all other nodes
}

#[derive(Debug)]
struct ParsedNode {
    flow: i32,
    links: HashSet<String>,
}

#[derive(Debug, Clone)]
struct AgentInfo {
    current_node: String,
    remaining_time: i32,
}

fn calculate_distances(pm: &HashMap<String, ParsedNode>) -> HashMap<String, ValveRoom> {
    let mut retval = HashMap::new();
    for node in pm.keys() {
        // this grows rapidly with size map, due to filter calls
        let mut distances_cloud: HashMap<String, i32> = HashMap::new();
        let mut distance = 0;
        distances_cloud.insert(node.to_string(), distance); // distance to itself is zero

        while distances_cloud.len() < pm.keys().len() {
            // all nodes on the edge of the cloud
            let edge_nodes: Vec<String> = distances_cloud
                .iter()
                .filter(|entry| *entry.1 == distance)
                .map(|entry| entry.0.to_string())
                .collect();
            distance += 1;
            for en in edge_nodes {
                for link in &pm.get(&en).unwrap().links {
                    if !distances_cloud.contains_key(link) {
                        // if key is present in distance cloud, a shorter path already exists
                        // if no insert it
                        distances_cloud.insert(link.to_string(), distance);
                    }
                }
            }
        }

        retval.insert(
            node.to_string(),
            ValveRoom {
                flow: pm.get(node).unwrap().flow,
                all_distances: distances_cloud,
            },
        );
    }
    // prune nodes with zero flow
    retval
}

fn main() {
    let ff = Regex::new(r"^Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z][A-Z](,\s*[A-Z][A-Z])*)$").unwrap();
    if let Ok(lines) = read_lines("../input.txt") {
        let mut parsed_map: HashMap<String, ParsedNode> = HashMap::new();
        for line in lines {
            if let Ok(as_str) = line {
                let captures = ff.captures(&as_str).unwrap();
                let from = captures.get(1).unwrap().as_str();
                let flow = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let to = captures.get(3).unwrap().as_str();
                let mut links = HashSet::new();
                for link in to.split(",").map(|s| s.trim()) {
                    links.insert(link.to_owned());
                }
                let parsed = ParsedNode {
                    flow: flow,
                    links: links,
                };
                parsed_map.insert(from.to_string(), parsed);
            }
        }
        let world = calculate_distances(&parsed_map);
        println!("{}", solve2(&world));
    }
}

fn _solve2(
    world: &HashMap<String, ValveRoom>,
    nv: &HashSet<String>,
    agents: &mut [AgentInfo; 2],
) -> i32 {
    // pick agent
    let agent = if agents[0].remaining_time > agents[1].remaining_time {
        0
    } else {
        1
    };
    let current_room = world.get(&agents[agent].current_node).unwrap();
    let mut room_value = if current_room.flow > 0 {
        agents[agent].remaining_time -= 1;
        current_room.flow * agents[agent].remaining_time
    } else {
        0
    };

    let mut best_solution = 0;
    if nv.len() > 0 {
        for nn in nv {
            let travel_cost = current_room.all_distances.get(nn).unwrap();
            if travel_cost < &agents[agent].remaining_time {
                let mut new_agents = agents.clone();
                new_agents[agent].remaining_time -= travel_cost;
                new_agents[agent].current_node = nn.to_string();
                let mut not_visited = nv.clone();
                not_visited.remove(nn);
                let sol = _solve2(world, &not_visited, &mut new_agents);
                best_solution = max(best_solution, sol);
            }
        }
        if agents[agent].remaining_time > 1 {
            let mut new_agents = agents.clone();
            new_agents[agent].remaining_time = 1;
            new_agents[agent].current_node = "AA".to_string();
            let sol = _solve2(world, nv, &mut new_agents);
            best_solution = max(best_solution, sol);
        }
    }
    if nv.len() == 0 || best_solution == 0 {
        // on last iteration other agent should open its valve if available
        let other_agent = if agent == 1 { 0 } else { 1 };
        let other_node = world.get(&agents[other_agent].current_node).unwrap();
        room_value += other_node.flow * (agents[other_agent].remaining_time - 1);
    }
    return best_solution + room_value;
}

fn solve2(world: &HashMap<String, ValveRoom>) -> i32 {
    let remaining_time = 26;
    let current_node = "AA";
    let worth_visiting: HashSet<String> = world
        .iter()
        .filter(|entry| entry.1.flow != 0)
        .map(|entry| entry.0.to_string())
        .collect();

    return _solve2(
        world,
        &worth_visiting,
        &mut [
            AgentInfo {
                remaining_time: remaining_time,
                current_node: current_node.to_string(),
            },
            AgentInfo {
                remaining_time: remaining_time,
                current_node: current_node.to_string(),
            },
        ],
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

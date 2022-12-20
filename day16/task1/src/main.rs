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
        println!("{}", solve1(&world));
    }
}

fn _solve1(world: &HashMap<String, ValveRoom>, rt: i32, cn: &String, nv: &HashSet<String>) -> i32 {
    let current_node = world.get(cn).unwrap();
    let mut remaining_time = rt;
    let flow_if_released = if current_node.flow > 0 {
        remaining_time -= 1;
        current_node.flow * remaining_time
    } else {
        0
    };

    let mut remaining = nv.clone();
    remaining.remove(cn);

    let mut max_flow_from_current = 0;
    for nn in &remaining {
        let time_expense = current_node.all_distances.get(nn).unwrap();
        if time_expense < &remaining_time {
            // visit that node
            let sol = _solve1(world, remaining_time - time_expense, &nn, &remaining);
            max_flow_from_current = max(max_flow_from_current, sol);
        }
    }
    return max_flow_from_current + flow_if_released;
}

fn solve1(world: &HashMap<String, ValveRoom>) -> i32 {
    let remaining_time = 30;
    let current_node = "AA";
    let worth_visiting: HashSet<String> = world
        .iter()
        .filter(|entry| entry.1.flow != 0)
        .map(|entry| entry.0.to_string())
        .collect();

    return _solve1(
        world,
        remaining_time,
        &current_node.to_string(),
        &worth_visiting,
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

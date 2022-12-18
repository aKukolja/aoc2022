use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use regex::Regex;


type Coord = (i32, i32);

struct World {
    sensors: HashMap<Coord, i32>,
    beacons: HashSet<Coord>
}

// y grows down x grows right
impl World {
    fn new() -> Self {
        Self {
            sensors: HashMap::new(),
            beacons: HashSet::new()
        }
    }
    fn add_sensor(&mut self, s: Coord, b: Coord) -> () {
        let distance = (s.1 - b.1).abs() + (s.0 - b.0).abs();
        self.sensors.insert(s, distance);
    }
    fn add_beacon(&mut self, c: Coord) -> () {
        self.beacons.insert(c);
    }
    fn solution(&self, y_coord: i32) -> usize {
        let mut solution: HashSet<Coord> = HashSet::new();
        for (sensor, coverage) in &self.sensors {
            if let Some((x_on_intersect, remainder)) = calculate(y_coord, sensor, *coverage) {
                // go from left to right
                for solution_x in (x_on_intersect-remainder)..(x_on_intersect+remainder+1) {
                    // TODO replace with set of ranges
                    let solution_coord = (solution_x, y_coord);
                    solution.insert(solution_coord);
                }
            }
        }
        for beacon in &self.beacons {
            solution.remove(beacon);
        }
        return solution.len();
    }
}

fn calculate(y: i32, sensor: &Coord, coverage: i32) -> Option<(i32, i32)> {
    let diff = (y - sensor.1).abs();
    if diff > coverage {
        return None;
    }
    return Some((sensor.0, coverage - diff));
}

fn main() {
    let ff = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    if let Ok(lines) = read_lines("../input.txt") {
        let mut world = World::new();
        for line in lines {
            if let Ok(as_str) = line {
                let captures = ff.captures(&as_str).unwrap();
                let sx = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let sy = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let bx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let by = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
                world.add_sensor((sx, sy), (bx, by));
                world.add_beacon((bx, by));
            }
        }
        // solution is union of all sensor coverage, then intersect with row
        let solution = world.solution(2000000);
        //let solution = world.solution(10);
        println!("{}", solution);
    }
}




fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



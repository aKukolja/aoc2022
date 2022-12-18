use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;


type Coord = (i32, i32);

const FACTOR: i64 = 4000000;
const LIMIT: i32 = 4000000;

struct Sensor {
    x: i32,
    y: i32,
    coverage: i32
}

impl Sensor {
    fn contains(&self, c: &Coord) -> bool {
        let distance = (self.x - c.0).abs() + (self.y - c.1).abs();
        return distance <= self.coverage;
    }
    /*
       A
      D.B
       C
     */
    fn check_border(&self, extra: usize, f: &dyn Fn(&Coord) -> bool) -> Option<Coord> {
        let offset = self.coverage + extra as i32;
        let a_x = self.x;
        let a_y = self.y - offset as i32;
        let b_x = self.x + offset as i32;
        let b_y = self.y;
        let c_x = self.x;
        let c_y = self.y + offset as i32;
        let d_x = self.x - offset as i32;
        let d_y = self.y;
        let direction_stop_start = [
            ((1, 1),   (b_x, b_y), (a_x, a_y)),
            ((-1, 1),  (c_x, c_y), (b_x, b_y)),
            ((-1, -1), (d_x, d_y), (c_x, c_y)),
            ((1, -1),  (a_x, a_y), (d_x, d_y))
        ];
        for (direction, stop, start) in direction_stop_start{
            let mut curr = start;
            while curr != stop {
                if curr.0 > LIMIT || curr.1 > LIMIT || curr.0 < 0 || curr.1 < 0 {
                    break;
                }
                if curr.0 < d_x || curr.0 > b_x || curr.1 < a_y ||curr.1 > c_y {
                    panic!("Out of bounds iteration");
                }
                let res = f(&curr);
                if res {
                    return Some(curr);
                }
                curr = (curr.0 + direction.0, curr.1 + direction.1);
            }
        }
        return None;
    }
}

struct World {
    sensors: Vec<Sensor>,
    beacons: HashSet<Coord>
}

// y grows down x grows right
impl World {
    fn new() -> Self {
        Self {
            sensors: Vec::new(),
            beacons: HashSet::new()
        }
    }
    fn add_sensor(&mut self, s: Coord, b: Coord) -> () {
        let distance = (s.1 - b.1).abs() + (s.0 - b.0).abs();
        let s = Sensor{
            x: s.0,
            y: s.1,
            coverage: distance
        };
        self.sensors.push(s);
    }
    fn add_beacon(&mut self, c: Coord) -> () {
        self.beacons.insert(c);
    }
    fn solution(&self) -> Option<Coord> {
        for i in 0..self.sensors.len() {
            let sensor = &self.sensors[i];
            let res = sensor.check_border(1, &|p| {

                for j in 0..self.sensors.len() {
                    if self.sensors[j].contains(p) {
                        return false;
                    }
                    if self.beacons.contains(p) {
                        return false;
                    }
                }
                return true;
            });
            
            if res.is_some() {
                return res;
            }

        }
        return None;
    }
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
        if let Some(solution) = world.solution() {
            println!("{:?}", solution);
            println!("{}", solution.0 as i64 * FACTOR + solution.1 as i64);
        } else {
            println!("No soultion");
        }
    }
}

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



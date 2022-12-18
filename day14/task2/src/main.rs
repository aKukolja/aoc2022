use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::max;

type Coord = (i32, i32);

const STARTX: i32 = 500;
const STARTY: i32 = 0;
// down, down left, down right
const DIRECTIONS: [Coord; 3] = [(0, 1), (-1, 1), (1, 1)];

enum WorldCell {
    Empty,
    Rock,
    Sand
}

struct World {
    state: HashMap<Coord, WorldCell>,
    maxy: i32,
    sand_count: usize
}

// y grows down x grows right
impl World {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
            maxy: i32::MIN,
            sand_count: 0
        }
    }
    fn is_taken(&self, c: Coord) -> bool {
        let floor = self.maxy + 2;
        if c.1 >= floor {
            return true;
        }
        if let Some(v) = self.state.get(&c) {
            match v {
                WorldCell::Empty => return false,
                _ => return true
            }
        }
        return false;
    }
    fn out_of_bounds(&self, c: Coord) -> bool {
        return c.1 > self.maxy;
    }
    fn insert_sand(&mut self) -> bool {
        let mut candidate_sand = (STARTX, STARTY);
        let mut prev: Option<Coord> = None;
        loop {
            if self.is_taken(candidate_sand) {
                break;
            } else {
                let mut found_next = false;
                for dd in DIRECTIONS {
                    let new_coord = (dd.0 + candidate_sand.0, dd.1 + candidate_sand.1);
                    if self.is_taken(new_coord) {
                        continue;
                    } else {
                        prev = Some(candidate_sand);
                        candidate_sand = new_coord;
                        found_next = true;
                        break;
                    }
                }
                if !found_next {
                    // all below are taken, put sand into candidate
                    self.state.insert(candidate_sand, WorldCell::Sand);
                    self.sand_count += 1;
                    return true;
                }
            }
        }

        match prev {
            None => {
                return false;
            }
            Some(cc) => {
                // TODO add to state
                self.state.insert(cc, WorldCell::Sand);
                self.sand_count += 1;
                return true;
            }
        }
    }
    fn add_wall(&mut self, start: Coord, end: Coord) {
        let (startx, starty) = start;
        let (endx, endy) = end;
        if startx == endx && starty == endy {
            panic!("{:?} {:?} must be different", start, end);
        }
        self.maxy = max(self.maxy, starty);
        self.maxy = max(self.maxy, endy);
        if startx != endx {
            // y equals, iterate over x
            let mut iter_x = if startx < endx {
                startx..endx+1
            } else {
                endx..startx+1
            };
            for xx in iter_x {
                let p = (xx, starty);
                self.state.insert(p, WorldCell::Rock);
            }
        } else {
            // x equals, iterate over y
            let mut iter_y = if starty < endy {
                starty..endy+1
            } else {
                endy..starty+1
            };
            for yy in iter_y {
                let p = (startx,  yy);
                self.state.insert(p, WorldCell::Rock);
            }
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut world = World::new();
        for line in lines {
            if let Ok(as_str) = line {
                let as_parts: Vec<Coord> = as_str.split(" -> ")
                    .map(|p| {
                        let mut parts = p.split(",");
                        let left = parts.next().unwrap().parse::<i32>().unwrap();
                        let right = parts.next().unwrap().parse::<i32>().unwrap();
                        (left, right)
                    })
                    .collect();
                for i in 0..as_parts.len() - 1 {
                    let start = as_parts[i];
                    let end = as_parts[i + 1];
                    world.add_wall(start, end);
                }
            }
        }
       
        loop {
            if !world.insert_sand() {
                break;
            }
        }
        println!("{}", world.sand_count);
    }
}




fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



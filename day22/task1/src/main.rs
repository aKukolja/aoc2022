use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// x, y; x grows left to right, y grows downwards
type Coord = (i32, i32);

// right, down, left, up
const DIRECTIONS: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type State = (Coord, usize);
#[derive(Debug)]
enum Instruction {
    Steps(usize),
    Rotate(bool),
}

fn next_direction(curr: usize, dd: bool) -> usize {
    if dd {
        return (curr + DIRECTIONS.len() + 1) % DIRECTIONS.len();
    } else {
        return (curr + DIRECTIONS.len() - 1) % DIRECTIONS.len();
    };
}

fn reverse_direction(curr: usize) -> usize {
    (curr + DIRECTIONS.len() / 2) % DIRECTIONS.len()
}

impl Instruction {
    fn from_str(s: &str) -> Vec<Instruction> {
        let mut retval = Vec::new();
        let mut start_index = 0;
        let mut end_index = 0;
        for c in s.chars() {
            // end index not contained
            let ss = &s[start_index..end_index];
            match c {
                '0'..='9' => {
                    end_index += 1;
                }
                'L' => {
                    retval.push(Instruction::Steps(ss.parse::<usize>().unwrap()));
                    end_index += 1;
                    start_index = end_index;
                    retval.push(Instruction::Rotate(false));
                }
                'R' => {
                    retval.push(Instruction::Steps(ss.parse::<usize>().unwrap()));
                    end_index += 1;
                    start_index = end_index;
                    retval.push(Instruction::Rotate(true));
                }
                _ => panic!("Invalid instruction"),
            }
        }
        if start_index != end_index {
            let ss = &s[start_index..end_index];
            if let Ok(val) = ss.parse::<usize>() {
                retval.push(Instruction::Steps(val));
            } else {
                match ss {
                    "R" => retval.push(Instruction::Rotate(true)),
                    "L" => retval.push(Instruction::Rotate(false)),
                    _ => panic!("Invalid instruction {}", ss),
                }
            }
        }
        return retval;
    }
}

#[derive(Default)]
struct World {
    space: HashMap<Coord, bool>, // false is free space
    max_x: i32,
    max_y: i32,
}

impl World {
    fn add_line(&mut self, line: &str) -> () {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    self.space.insert((i as i32, self.max_y), false);
                }
                '#' => {
                    self.space.insert((i as i32, self.max_y), true);
                }
                _ => (),
            }
            self.max_x = self.max_x.max(i as i32);
        }
        self.max_y += 1;
    }

    fn solve(&self, instructions: &str) -> i32 {
        let start_x: i32 = (0..)
            .into_iter()
            .find_map(|i| self.space.get(&(i, 0)).map(|_| i))
            .unwrap();
        let mut current = ((start_x, 0), 0);
        println!("current: {:?}", current);

        for i in Instruction::from_str(instructions) {
            self.apply(&mut current, &i);
        }

        println!("{:?}", current.0);

        return 1000 * (current.0 .1 + 1) + 4 * (current.0 .0 + 1) + current.1 as i32;
    }

    fn apply(&self, current: &mut State, i: &Instruction) -> () {
        let current_dir = DIRECTIONS[current.1];
        match i {
            Instruction::Steps(sc) => {
                for _ in 0..*sc {
                    let next_coord = (current.0 .0 + current_dir.0, current.0 .1 + current_dir.1);
                    if let Some(next_space) = self.space.get(&next_coord) {
                        if *next_space {
                            return;
                        }
                        current.0 .0 = next_coord.0;
                        current.0 .1 = next_coord.1; // move available step
                    } else {
                        // TODO wrap around
                        self.wrap_around(current);
                    }
                }
            }
            Instruction::Rotate(dd) => {
                current.1 = next_direction(current.1, *dd);
            }
        }
    }

    fn wrap_around(&self, current: &mut State) -> bool {
        // true if shoud stop movement
        let direction = DIRECTIONS[reverse_direction(current.1)];
        let mut wac = (current.0 .0, current.0 .1);
        loop {
            let next_coord = (wac.0 + direction.0, wac.1 + direction.1);
            if let Some(_) = self.space.get(&next_coord) {
                wac.0 = next_coord.0;
                wac.1 = next_coord.1;
            } else {
                if !self.space.get(&wac).unwrap() {
                    current.0 .0 = wac.0;
                    current.0 .1 = wac.1;
                    return false;
                }
                return true;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut parse_directions = false;
    let mut w = World::default();

    for line in reader.lines() {
        let ll = line?;
        if ll.len() == 0 {
            parse_directions = true;
        } else if parse_directions {
            // TODO
            println!("{}", w.solve(&ll));
        } else {
            w.add_line(&ll);
        }
    }

    return Ok(());
}

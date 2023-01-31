use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// x, y; x grows left to right, y grows downwards
type Coord = (i32, i32);

// right, down, left, up
#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn next_direction(&self, dd: bool) -> Direction {
        match (self, dd) {
            (Direction::Right, true) => Direction::Down,
            (Direction::Down, true) => Direction::Left,
            (Direction::Left, true) => Direction::Up,
            (Direction::Up, true) => Direction::Right,
            (Direction::Right, false) => Direction::Up,
            (Direction::Down, false) => Direction::Right,
            (Direction::Left, false) => Direction::Down,
            (Direction::Up, false) => Direction::Left,
        }
    }
    fn next_coord(&self, cc: Coord) -> Coord {
        let dd = match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        };
        (cc.0 + dd.0, cc.1 + dd.1)
    }
}

/*
   Faces:
       1
   6 5 2
       3 4
*/
const SIZE: usize = 50;
const FACES: usize = 6;
type Face = [[bool; SIZE]; SIZE];
type Cube = [Face; FACES];

// face, coord, direction
type State = (usize, Coord, Direction);

#[derive(Debug)]
enum Instruction {
    Steps(usize),
    Rotate(bool),
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

struct World {
    space: Cube,
    max_y: usize,
}

fn global_to_face_coord(x: i32, y: i32) -> (usize, Coord) {
    let xs = x / SIZE as i32;
    let ys = y / SIZE as i32;
    let coord = (x % SIZE as i32, y % SIZE as i32);
    match (xs, ys) {
        (1, 0) => (0, coord),
        (2, 0) => (1, coord),
        (1, 1) => (2, coord),
        (0, 2) => (3, coord),
        (1, 2) => (4, coord),
        (0, 3) => (5, coord),
        _ => panic!(
            "Invalid coordinate: input({:?}), xs, ys {:?}",
            (x, y),
            (xs, ys)
        ),
    }
}

fn state_to_global(c: &State) -> Coord {
    let face = c.0;
    let x = c.1 .0;
    let y = c.1 .1;
    let ss = SIZE as i32;
    match face {
        0 => (ss + x, y),
        1 => (2 * ss + x, y),
        2 => (ss + x, ss + y),
        3 => (x, 2 * ss + y),
        4 => (ss + x, 2 * ss + y),
        5 => (x, 3 * ss + y),
        _ => panic!("Invalid coordinate in state: {:?}", c),
    }
}

impl World {
    fn new() -> Self {
        Self {
            space: [[[false; SIZE]; SIZE]; 6],
            max_y: 0,
        }
    }
    fn add_line(&mut self, line: &str) -> () {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    let (face, (x, y)) = global_to_face_coord(i as i32, self.max_y as i32);
                    self.space[face][y as usize][x as usize] = false;
                }
                '#' => {
                    let (face, (x, y)) = global_to_face_coord(i as i32, self.max_y as i32);
                    self.space[face][y as usize][x as usize] = true;
                }
                _ => (),
            }
        }
        self.max_y += 1;
    }

    fn solve(&self, instructions: &str) -> i32 {
        let mut state = (0, (0, 0), Direction::Right);

        for i in Instruction::from_str(instructions) {
            self.apply(&mut state, &i);
        }

        println!("End state {:?}", state);

        let (x, y) = state_to_global(&state);
        println!("Global xy =  {} {}", x, y);
        let direction_val = match state.2 {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        return 1000 * (y + 1) + 4 * (x + 1) + direction_val;
    }

    fn next_state(&self, current: &State) -> State {
        let nc = current.2.next_coord(current.1);

        if !(nc.0 < 0 || nc.0 >= SIZE as i32 || nc.1 < 0 || nc.1 >= SIZE as i32) {
            return (current.0, nc, current.2);
        }
        let x = current.1 .0;
        let y = current.1 .1;
        let mm = SIZE as i32 - 1;
        // next step changes current face
        match (current.0, current.2) {
            (0, Direction::Right) => (1, (0, y), Direction::Right),
            (0, Direction::Down) => (2, (x, 0), Direction::Down),
            (0, Direction::Left) => (3, (0, mm - y), Direction::Right),
            (0, Direction::Up) => (5, (0, x), Direction::Right),

            (1, Direction::Right) => (4, (mm, mm - y), Direction::Left),
            (1, Direction::Down) => (2, (mm, x), Direction::Left),
            (1, Direction::Left) => (0, (mm, y), Direction::Left),
            (1, Direction::Up) => (5, (x, mm), Direction::Up),

            (2, Direction::Right) => (1, (y, mm), Direction::Up),
            (2, Direction::Down) => (4, (x, 0), Direction::Down),
            (2, Direction::Left) => (3, (y, 0), Direction::Down),
            (2, Direction::Up) => (0, (x, mm), Direction::Up),

            (3, Direction::Right) => (4, (0, y), Direction::Right),
            (3, Direction::Down) => (5, (x, 0), Direction::Down),
            (3, Direction::Left) => (0, (0, mm - y), Direction::Right),
            (3, Direction::Up) => (2, (0, x), Direction::Right),

            (4, Direction::Right) => (1, (mm, mm - y), Direction::Left),
            (4, Direction::Down) => (5, (mm, x), Direction::Left),
            (4, Direction::Left) => (3, (mm, y), Direction::Left),
            (4, Direction::Up) => (2, (x, mm), Direction::Up),

            (5, Direction::Right) => (4, (y, mm), Direction::Up),
            (5, Direction::Down) => (1, (x, 0), Direction::Down),
            (5, Direction::Left) => (0, (y, 0), Direction::Down),
            (5, Direction::Up) => (3, (x, mm), Direction::Up),

            _ => panic!("Invalid state"),
        }
    }

    fn apply_step(&self, sc: usize, current: &mut State) -> () {
        //let current_dir: Coord = current.2.into();
        for _ in 0..sc {
            let ns = self.next_state(current);
            if self.space[ns.0][ns.1 .1 as usize][ns.1 .0 as usize] {
                return;
            } else {
                current.0 = ns.0;
                current.1 = ns.1;
                current.2 = ns.2;
            }
        }
    }

    fn apply(&self, current: &mut State, i: &Instruction) -> () {
        match i {
            Instruction::Steps(sc) => {
                self.apply_step(*sc, current);
            }
            Instruction::Rotate(dd) => {
                current.2 = current.2.next_direction(*dd);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut parse_directions = false;
    let mut w = World::new();

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

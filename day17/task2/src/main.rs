use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ITER_LIMIT: u64 = 1000000000000;
const MAX_HEIGHT: u64 = 100000; // should be high enough
const BUFF_SIZE: u64 = MAX_HEIGHT / 2;

enum Direction {
    Left,
    Right,
}

type Rock = [u8; 4];

#[rustfmt::skip]
const LINE: Rock = [
//    /- this bit is extra, wall
//    |
    0b00111100, // index 0 is lower
    0b00000000, 
    0b00000000, 
    0b00000000,
];
#[rustfmt::skip]
const CROSS: Rock = [
    0b00001000, 
    0b00011100, 
    0b00001000, 
    0b00000000
];
#[rustfmt::skip]
const EL: Rock = [
    0b00011100, // this is the left side
    0b00010000, 
    0b00010000, 
    0b00000000,
];
#[rustfmt::skip]
const VERTICAL: Rock = [
    0b00000100, 
    0b00000100, 
    0b00000100, 
    0b00000100
];
#[rustfmt::skip]
const SQUARE: Rock = [
    0b00001100, 
    0b00001100, 
    0b00000000, 
    0b00000000
];

// TODO: i bet task 2 will increase iteration number and solution is to remove slice of the cave which is blocked off
struct Cave {
    space: [u8; MAX_HEIGHT as usize], // rocks will be falling upward in this case
    max_height: u64,
    nb_baked: u64,
}

impl Cave {
    fn new() -> Self {
        Self {
            space: [0; MAX_HEIGHT as usize],
            max_height: 0,
            nb_baked: 0,
        }
    }

    fn shift_rock(&self, rock_height: u64, rock: &mut Rock, direction: &Direction) -> bool {
        // shift right or left
        let colapsed = rock[0] | rock[1] | rock[2] | rock[3];
        match direction {
            Direction::Right => {
                // direction right means shift left in array
                if colapsed & 0b01000000 == 0 {
                    // shifting left will not crash
                    rock.iter_mut().for_each(|s| *s <<= 1);
                }
            }
            Direction::Left => {
                // direction left means shift right in array
                if colapsed & 0b00000001 == 0 {
                    // shifting right will not crash
                    rock.iter_mut().for_each(|s| *s >>= 1);
                }
            }
        }

        // collision here means a shift back
        if self.rock_colide(rock_height, rock) {
            match direction {
                Direction::Right => {
                    rock.iter_mut().for_each(|s| *s >>= 1);
                }
                Direction::Left => {
                    rock.iter_mut().for_each(|s| *s <<= 1);
                }
            }
        }

        // shift down due to gravity, if collision, return and bake
        if rock_height == 0 {
            return true;
        }
        // collision on lower height means that current height is acceptable to bake
        return self.rock_colide(rock_height - 1, rock);
    }

    fn rock_colide(&self, rock_height: u64, rock: &Rock) -> bool {
        let window = &self.space[rock_height as usize..rock_height as usize + 4];
        for i in 0..4 {
            if window[i] & rock[i] != 0 {
                return true;
            }
        }
        return false;
    }

    fn settle_rock(
        &mut self,
        rock: &mut Rock,
        directions: &mut dyn Iterator<Item = (usize, &Direction)>,
    ) -> () {
        let mut rock_height = self.max_height + 3;
        loop {
            let (_, direction) = directions.next().unwrap();
            if self.shift_rock(rock_height, rock, direction) {
                let bake_height = self.bake_in(rock_height, rock);
                let candidate_height = rock_height + bake_height;
                self.max_height = max(candidate_height, self.max_height);
                return;
            }
            rock_height -= 1;
        }
    }

    fn bake_in(&mut self, rock_height: u64, rock: &Rock) -> u64 {
        let window = &mut self.space[rock_height as usize..rock_height as usize + 4];
        let mut retval = 0;
        for i in 0..4 {
            window[i] = window[i] | rock[i];
            if window[i] != 0 {
                retval += 1;
            }
        }
        self.nb_baked += 1;
        return retval;
    }
}

fn spinner(wind: Vec<Direction>) -> u64 {
    let mut rocks = [LINE, CROSS, EL, VERTICAL, SQUARE]
        .iter()
        .enumerate()
        .cycle();
    let mut current_rock: Rock = [0; 4];
    let mut cave = Cave::new();
    let mut wind_iter = wind.iter().enumerate().cycle().peekable();
    let mut history: HashMap<(u8, usize, usize), u64> = HashMap::new();
    let mut heights: [u64; MAX_HEIGHT as usize] = [0; MAX_HEIGHT as usize];

    for rock_to_place in 0..BUFF_SIZE {
        let (rock_index, rock) = rocks.next().unwrap();
        let (starting_wind_index, _) = wind_iter.peek().unwrap();
        if let Some(old_rock_to_place) = history.insert(
            (
                cave.space[cave.max_height as usize],
                rock_index,
                *starting_wind_index,
            ),
            rock_to_place,
        ) {
            //
            let q = (ITER_LIMIT - old_rock_to_place) / (rock_to_place - old_rock_to_place);
            let r = (ITER_LIMIT - old_rock_to_place) % (rock_to_place - old_rock_to_place);
            return heights[(old_rock_to_place + r) as usize]
                + q * (cave.max_height - heights[old_rock_to_place as usize]);
        }

        current_rock.copy_from_slice(rock); // copy faster than allocation
        cave.settle_rock(&mut current_rock, &mut wind_iter);
        heights[rock_to_place as usize] = cave.max_height;
    }
    panic!("Solution not found");
    //return cave.max_height;
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines {
            if let Ok(as_str) = line {
                let wind: Vec<Direction> = as_str
                    .chars()
                    .map(|c| {
                        if c == '<' {
                            Direction::Left
                        } else if c == '>' {
                            Direction::Right
                        } else {
                            panic!("Invalid direction")
                        }
                    })
                    .collect();
                println!("{}", spinner(wind));
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

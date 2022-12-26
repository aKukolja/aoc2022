use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

type Cube = (i32, i32, i32);

const DIRECTIONS: [Cube; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[derive(Default)]
struct Space {
    space: HashSet<Cube>,
    sides: usize,
}

impl Space {
    fn add_cube(&mut self, cube: Cube) -> () {
        self.sides += 6;
        for d in DIRECTIONS {
            let nc = (d.0 + cube.0, d.1 + cube.1, d.2 + cube.2);
            if self.space.contains(&nc) {
                self.sides -= 2;
            }
        }
        self.space.insert(cube);
    }
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut space = Space::default();
        for line in lines {
            if let Ok(as_str) = line {
                let as_nums = as_str
                    .split(",")
                    .into_iter()
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                space.add_cube((as_nums[0], as_nums[1], as_nums[2]));
            }
        }
        println!("{}", space.sides);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

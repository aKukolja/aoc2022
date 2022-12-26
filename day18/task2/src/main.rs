use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::cmp::{min, max};

type Cube = (i32, i32, i32);

const DIRECTIONS: [Cube; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

struct Space {
    droplet: HashSet<Cube>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Space {
    fn new() -> Self {
        Self {
            droplet: HashSet::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN
        }
    }
    fn add_cube(&mut self, cube: Cube) -> () {
        self.min_x = min(self.min_x, cube.0);
        self.max_x = max(self.max_x, cube.0);
        self.min_y = min(self.min_y, cube.1);
        self.max_y = max(self.max_y, cube.1);
        self.min_z = min(self.min_z, cube.2);
        self.max_z = max(self.max_z, cube.2);
        self.droplet.insert(cube);
    }

    fn out_of_bound(&self, c: &Cube) -> bool {
        if !(self.min_x <= c.0 && c.0 <= self.max_x) {
            return true;
        }
        if !(self.min_y <= c.1 && c.1 <= self.max_y) {
            return true;
        }
        if !(self.min_z <= c.2 && c.2 <= self.max_z) {
            return true;
        }
        return false;
    }
    fn solve(&mut self) -> usize {
        let mut retval = 0;
        self.min_x -= 2;
        self.max_x += 2;
        self.min_y -= 2;
        self.max_y += 2;
        self.min_z -= 2;
        self.max_z += 2;
        
        let mut visited: HashSet<Cube> = HashSet::new();
        let mut q: Vec<Cube> = Vec::new();
        q.push((self.min_x, self.min_y, self.min_z));

        while q.len() != 0 {
            let current: Cube = q.pop().unwrap();
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for d in DIRECTIONS {
                let nc = (d.0 + current.0, d.1 + current.1, d.2 + current.2);
                if self.out_of_bound(&nc) || visited.contains(&nc){
                    continue;
                }
                if self.droplet.contains(&nc) {
                    retval += 1;  // not putting in steam
                } else {
                    q.push(nc);
                }
            }

        }

        return retval;
    }
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut space = Space::new();
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
        println!("{}", space.solve());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

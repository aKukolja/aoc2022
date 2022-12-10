use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

struct Grid {
    grid_raw: Vec<Vec<i8>>,
    height: usize,
    width: usize,
}

const MAX_TREE_SIZE: i8 = 9;

impl Grid {
    fn new() -> Self {
        Self {
            grid_raw: Vec::new(),
            width: 0,
            height: 0
        }
    }
    fn add_value(&mut self, line: String) -> () {
        let val: Vec<i8> = line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
        self.width = val.len();
        self.grid_raw.push(val);
        self.height += 1
    }

    fn scene(&self, ri: usize, ci: usize) -> usize {
        let candidate = self.grid_raw[ri][ci];
        // left
        let mut left_count = 0;
        for c in (0..ci).rev() {
            left_count += 1;
            if self.grid_raw[ri][c] >= candidate {
                break;
            }
        }
        // right
        let mut right_count = 0;
        for c in (ci+1)..self.width {
            right_count += 1;
            if self.grid_raw[ri][c] >= candidate {
                break;
            }
        }
        // top
        let mut top_count = 0;
        for r in (0..ri).rev() {
            top_count += 1;
            if self.grid_raw[r][ci] >= candidate {
                break;
            }
        }
        // down
        let mut bottom_count = 0;
        for r in (ri+1)..self.height {
            bottom_count += 1;
            if self.grid_raw[r][ci] >= candidate {
                break;
            }
        }
        return right_count * top_count * left_count * bottom_count;
    }

    fn solve2(& self) -> usize {
        let mut best = 0;
        for i in 0..self.height {
            for j in 0..self.width { 
                best = max(best, self.scene(i, j));
            }
        }
        return best;
    }

}


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut grid = Grid::new();
        for line in lines {
            if let Ok(as_str) = line {
                grid.add_value(as_str);
            }
        }
        let result = grid.solve2();
        println!("{}", result);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





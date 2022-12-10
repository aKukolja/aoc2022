use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Grid {
    grid_raw: Vec<Vec<i8>>,
    grid_visible: Vec<Vec<bool>>  // false if not visible
}

const MAX_TREE_SIZE: i8 = 9;

impl Grid {
    fn new() -> Self {
        Self {
            grid_raw: Vec::new(),
            grid_visible: Vec::new()
        }
    }
    fn add_value(&mut self, line: String) -> () {
        let val: Vec<i8> = line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
        if self.grid_raw.len() == 1 {
            let val_b: Vec<bool> = vec![false; val.len()];
            self.grid_visible.push(val_b);
        } else {
            let val_b: Vec<bool> = vec![false; val.len()];
            self.grid_visible.push(val_b);
        }
        self.grid_raw.push(val);
    }
    fn solve(&mut self) -> usize {
        let height = self.grid_raw.len();
        let width = self.grid_raw[0].len();

        // go row by row / left and right
        for i in 0..height {
            let mut left_max = -1;
            let row = &self.grid_raw[i];
            let bool_row = &mut self.grid_visible[i];
            for j in 0..width {
                // left to right
                let right = row[j];
                if left_max < right {
                    bool_row[j] = true;  // this tree is visible
                    left_max = right;
                }
                if left_max == MAX_TREE_SIZE {
                    break;  // remaining trees wont be wisible from left side
                }
            }
            let mut right_max = -1;
            for j in (1..width+1).rev() {
                let left = row[j-1];
                if right_max < left {
                    bool_row[j-1] = true;  // this tree is visible
                    right_max = left;
                } 
                if right_max == MAX_TREE_SIZE {
                    break;  // remaining trees wont be wisible from right side
                }
            }
        }

        for column in 0..width {
            // top to bottom
            let mut top_max = -1;
            for br_index in 0..height {
                let bottom = self.grid_raw[br_index][column];
                if top_max < bottom {
                    self.grid_visible[br_index][column] = true;
                    top_max = bottom;
                }
                if top_max == MAX_TREE_SIZE {
                    break;
                }
            }
            // bottom to top
            let mut bottom_max = -1;
            for tr_index in (1..height+1).rev() {
                let top = self.grid_raw[tr_index-1][column];
                if bottom_max < top {
                    self.grid_visible[tr_index-1][column] = true;
                    bottom_max = top;
                }
                if bottom_max == MAX_TREE_SIZE {
                    break;
                }
            }
        }

        let invisible: usize = self.grid_visible.iter().map(|r| { r.iter().filter(|v| !*v).count() }).sum();

        return height * width - invisible;
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
        let result = grid.solve();
        println!("{}", result);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{VecDeque, HashMap};

const SIZE_MAX: usize = 4;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut window: VecDeque<char> = VecDeque::new();
        let mut count: HashMap<char, usize> = HashMap::new();
        for line in lines {
            if let Ok(as_str) = line {
                for (i, c) in as_str.chars().enumerate() {
                    window.push_back(c);
                    *count.entry(c).or_insert(0) += 1;
                    if window.len() > SIZE_MAX {
                        let removal = window.pop_front().unwrap();
                        *count.entry(removal).or_insert(0) -= 1;
                    }
                    if window.len() == SIZE_MAX {
                        let mut complete = true;
                        for c in &window {
                            if *count.entry(*c).or_insert(0) != 1 as usize{
                                complete = false;
                                break;
                            }
                        }
                        if complete {
                            println!("{}", i + 1);
                            return;
                        }
                    }
                }
            }
        }
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





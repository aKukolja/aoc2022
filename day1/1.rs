use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: i32 = 0;
        let mut maxx: i32 = 0;
        for line in lines {
            if let Ok(as_str) = line {
                match as_str.trim().parse::<i32>() {
                    Ok(as_int) => sum += as_int,
                    Err(_e) => {
                        if sum > maxx {
                            maxx = sum;
                        }
                        sum = 0;
                    }
                }
            }
        }
        if sum > maxx {
            maxx = sum;
        }
        println!("{}", maxx);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
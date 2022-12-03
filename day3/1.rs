use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(as_str) = line {
                let mut left = [0; 26*2];
                let mut right = [0; 26*2];

                let mut i = 0;
                for c in as_str.chars() {
                    let arr = if i < as_str.len() / 2 {
                        &mut left
                    } else {
                        &mut right
                    };
                    let ind = if c.is_lowercase() {
                        c as usize - 'a' as usize
                    } else {
                        c as usize - 'A' as usize + 26
                    };
                    arr[ind] += 1;
                    i += 1;
                }
                
                for i in 0..52 {
                    if left[i] >= 1 && right[i] >= 1 {
                        sum += i + 1;
                    }
                }
            }
        }
        println!("{}", sum);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn add_sum(sum: i32, arr: &mut [i32]) {
    let mut i = 0;
    while i < arr.len() {
        if sum > arr[i] {
            break;
        }
        i += 1;
    }
    let mut hold = sum;
    while i < arr.len() {
        let tmp = arr[i];
        arr[i] = hold;
        hold = tmp;
        if i == arr.len() - 1 {
            break;
        }
        i += 1;
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: i32 = 0;
        let mut maxx = [0, 0, 0];
        for line in lines {
            if let Ok(as_str) = line {
                match as_str.trim().parse::<i32>() {
                    Ok(as_int) => sum += as_int,
                    Err(_e) => {
                        add_sum(sum, &mut maxx);
                        sum = 0;
                    }
                }
            }
        }
        add_sum(sum, &mut maxx);
        println!("{}", maxx.iter().sum::<i32>());
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

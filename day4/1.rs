use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn contains(l: (i32, i32), r: (i32, i32)) -> bool {
    let (ls, le) = l;
    let (rs, re) = r;
    return (ls <= rs && re <= le) || (rs <= ls && le <= re);
}

fn parse_range(candidate: &str) -> Option<(i32, i32)> {
    let as_split: Vec<&str> = candidate.split("-").collect();
    if as_split.len() == 2 {
        if let (Ok(x), Ok(y)) = (as_split[0].parse::<i32>(), as_split[1].parse::<i32>()) {
            return Some((x, y));
        }
    }
    println!("Can't parse {}", candidate);
    return None;
}

fn parse_line(line: &str) -> bool {
    let s: Vec<&str> = line.split(",").collect();
    if s.len() == 2 {
        if let (Some(l), Some(r)) = (parse_range(s[0]), parse_range(s[1])) {
            return contains(l, r);
        }

    }
    println!("Cant match {}", line);
    return false;
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(as_str) = line {
                if parse_line(&as_str) { 
                    sum += 1;
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
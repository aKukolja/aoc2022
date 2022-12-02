use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
rock -> 1
paper -> 2
scissors -> 3

A -> rock
B -> paper
C -> scissors

X -> lose
Y -> tie
Z -> win
*/

const SCORES: [[i32; 3]; 3] = [
// i lose tie win
    [3+0, 1+3, 2+6],  // opp picked rock
    [1+0, 2+3, 3+6],  // opp picked paper
    [2+0, 3+3, 1+6]   // opp picked scissors
];

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut score = 0;
        for line in lines {
            if let Ok(as_str) = line {
                let opponent = as_str.chars().nth(0).unwrap() as usize - 'A' as usize;
                let me = as_str.chars().nth(2).unwrap() as usize - 'X' as usize;
                score += SCORES[opponent][me];
            }
        }
        println!("Score: {}", score);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
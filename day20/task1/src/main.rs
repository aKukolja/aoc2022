use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// original index to value
type Node = (usize, i32);

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut lls: Vec<Node> = reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .enumerate()
        .collect();

    for original_index in 0..lls.len() {
        let current_index = lls.iter().position(|(orig, _)| *orig == original_index).unwrap();
        let popped = lls.remove(current_index);
        let next_index = (current_index as i32 + popped.1).rem_euclid(lls.len() as i32) as usize;
        lls.insert(next_index, popped);

        //println!("{:?}", lls.iter().map(|n| n.1).collect::<Vec<i32>>());
        //println!("");
    }

    let zero_index = lls.iter().position(|(_, val)| 0 == *val).unwrap();
    let solution: i32 = [1000, 2000, 3000].into_iter().map(|n| {
        let si = (n + zero_index) % lls.len();
        lls[si].1
    }).sum();

    println!("{}", solution);

    return Ok(());
}

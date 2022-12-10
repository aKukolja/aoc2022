use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


// x, y
type Point = (i32, i32);


struct Rope {
    head: Point,
    tail: Point,
    tail_visited: HashSet<Point>,
}

enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

impl Rope {
    fn new() -> Self {
        let mut s = Self {
            head: (0, 0),
            tail: (0, 0),
            tail_visited: HashSet::new()
        };
        s.tail_visited.insert((0,0));
        return s;
    }
    fn handle_diff(&mut self) -> () {
        let diff_x = self.head.0 - self.tail.0;
        let diff_y = self.head.1 - self.tail.1;
        let mut tail_x = self.tail.0;
        let mut tail_y = self.tail.1;
        if diff_x.abs() > 1 {
            if diff_x > 0 {  // head is to the right
                tail_x = self.head.0 - 1;
                tail_y = self.head.1;
            } else {
                tail_x = self.head.0 + 1;
                tail_y = self.head.1;
            }
        }
        if diff_y.abs() > 1 {
            if diff_y > 0 {  // head is above tail
                tail_y = self.head.1 - 1;
                tail_x = self.head.0;
            } else {
                tail_y = self.head.1 + 1;
                tail_x = self.head.0;
            }
        }
        self.tail = (tail_x, tail_y);
        self.tail_visited.insert((tail_x, tail_y));
    }
    fn down(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.head = (self.head.0, self.head.1 - 1);
            self.handle_diff();
        }
    }
    fn left(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.head = (self.head.0 - 1, self.head.1);
            self.handle_diff();
        }
    }
    fn right(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.head = (self.head.0 + 1, self.head.1);
            self.handle_diff();
        }
    }
    fn up(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.head = (self.head.0, self.head.1 + 1);
            self.handle_diff();
        }
    }
    fn step(&mut self, d: Direction) -> () {
        match d {
            Direction::Up(i) => self.up(i),
            Direction::Down(i) => self.down(i),
            Direction::Left(i) => self.left(i),
            Direction::Right(i) => self.right(i),
        }
    }
    fn position_count(&self) -> usize {
        return self.tail_visited.len();
    }
}

fn parse_direction(line: &str) -> Option<Direction> {
    let left = &line[0..2];
    let right = &line[2..];

    let val = right.parse::<usize>();
    if  val.is_err() {
        return None
    }
    
    match left {
        "U " => Some(Direction::Up(val.unwrap())),
        "D " => Some(Direction::Down(val.unwrap())),
        "L " => Some(Direction::Left(val.unwrap())),
        "R " => Some(Direction::Right(val.unwrap())),
        _ => None
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut rope = Rope::new();
        for line in lines {
            if let Ok(as_str) = line {
                if let Some(direction) = parse_direction(&as_str) {
                    rope.step(direction);
                }
            }
        }
        let result = rope.position_count();
        println!("{}", result);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


// x, y
type Point = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

fn clamp(v: i32) -> i32 {
    if v > 0 {
        return 1;
    }
    if v < 0 {
        return -1;
    }
    return v
}

fn handle_diff(me: Point, head: Point) -> Point {
    if (head.0 == me.0 && (head.1 - me.1).abs() == 1) ||  // on same x, y varies by 1
        (head.1 == me.1 && (head.0 - me.0).abs() == 1) {   // on same y, x varies by 1
            return (me.0, me.1);  // change nothinmg
    } else if (head.0 - me.0).abs() == 1 && (head.1 - me.1).abs() == 1 {
        return (me.0, me.1);  // change nothing, diagonal touch
    } else {
        if head.0 != me.0 && head.1 != me.1 {  // moving diagonaly
            let x = me.0 + clamp(head.0 - me.0);
            let y = me.1 + clamp(head.1 - me.1);
            return (x, y);
        } else {
            if head.0 == me.0 {
                let y = me.1 + clamp(head.1 - me.1);
                return (me.0, y);
            } else {
                let x = me.0 + clamp(head.0 - me.0);
                return (x, me.1);
            }
        }
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

struct Rope {
    knots: Vec<Point>,
    tail_visited: HashSet<Point>
}

impl Rope {
    fn new() -> Rope {
        let mut s = Self {
            knots: vec![Point::default(); 10],
            tail_visited: HashSet::new()
        };
        s.tail_visited.insert((0, 0));
        return s;
    }

    fn position_count(&self) -> usize {
        return self.tail_visited.len();
    }

    fn step(&mut self, d: Direction) -> () {
        match d {
            Direction::Up(i) => self.up(i),
            Direction::Down(i) => self.down(i),
            Direction::Left(i) => self.left(i),
            Direction::Right(i) => self.right(i),
        }
    }

    fn move_knots(&mut self) -> () {
        for i in 1..self.knots.len() {
            let head = self.knots[i-1];
            let tail = self.knots[i];
            let tmp = handle_diff(tail, head);
            self.knots[i] = tmp;
        }
    }

    fn down(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.knots[0] = (self.knots[0].0, self.knots[0].1 -1);
            self.move_knots();
            self.tail_visited.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn left(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.knots[0] = (self.knots[0].0 - 1, self.knots[0].1);
            self.move_knots();
            self.tail_visited.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn right(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.knots[0] = (self.knots[0].0 + 1, self.knots[0].1);
            self.move_knots();
            self.tail_visited.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn up(&mut self, c: usize) -> () {
        for _ in 0..c {
            self.knots[0] = (self.knots[0].0, self.knots[0].1 + 1);
            self.move_knots();
            self.tail_visited.insert(self.knots[self.knots.len() - 1]);
        }
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





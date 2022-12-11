use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Computor {
    x: i32,
    pc: usize,
    signal: i32,
}


enum Instruction {
    Noop,
    AddX(i32)
}

impl Computor {
    fn new() -> Self {
        Self {
            x: 1,
            pc: 0,
            signal: 0
        }
    }
    fn execute_instruction(&mut self, i: Instruction) -> () {
        match i {
            Instruction::Noop => {
                self.pc += 1;
                self.recheck();
            }
            Instruction::AddX(x) => {
                self.pc += 1;
                self.recheck();
                self.pc += 1;
                self.recheck();
                self.x += x;
            }
        }
    }
    fn recheck(&mut self) -> () {
        if self.pc == 20 || self.pc == 60 || self.pc == 100 || self.pc == 140 || self.pc == 180 || 
        self.pc == 220 {
            self.signal += self.x * self.pc as i32;
        }
    }
 }

fn parse_instruction(line: &str) -> Option<Instruction> {
    if line.starts_with("noop") {
        return Some(Instruction::Noop);
    }
    if ! line.starts_with("addx ") {
        return None;
    }
    let right = &line[5..];

    let val = right.parse::<i32>();
    if  val.is_err() {
        return None
    }

    return Some(Instruction::AddX(val.unwrap()));
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut cpu = Computor::new();
        for line in lines {
            if let Ok(as_str) = line {
                if let Some(i) = parse_instruction(&as_str) {
                    cpu.execute_instruction(i);
                }
            }
        }
        println!("{}", cpu.signal);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





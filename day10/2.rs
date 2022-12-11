use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const WIDTH: u32 = 40;
const HEIHGT: u32 = 6;


struct Device {
    x: i32,
    pc: u32,
    framebuffer: [[char ; WIDTH as usize] ; HEIHGT as usize]
}


enum Instruction {
    Noop,
    AddX(i32)
}

impl Device {
    fn new() -> Self {
        Self {
            x: 1,
            pc: 0,
            framebuffer: [['.'; WIDTH as usize]; HEIHGT as usize]
        }
    }
    fn execute_instruction(&mut self, i: Instruction) -> () {
        match i {
            Instruction::Noop => {
                self.draw();
                self.pc += 1;
            }
            Instruction::AddX(x) => {
                self.draw();
                self.pc += 1;
                self.draw();
                self.pc += 1;
                self.x += x;
            }
        }
    }
    fn draw(&mut self) -> () {
        if self.pc >= WIDTH * HEIHGT {
            return;
        }
        let ci = self.pc % WIDTH;
        let ri = self.pc / WIDTH;
        if (ci as i32 - self.x).abs() <= 1 {
            self.framebuffer[ri as usize][ci as usize] = '#';
        } else {
            self.framebuffer[ri as usize][ci as usize] = '.';
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
        let mut cpu = Device::new();
        for line in lines {
            if let Ok(as_str) = line {
                if let Some(i) = parse_instruction(&as_str) {
                    cpu.execute_instruction(i);
                }
            }
        }
        for ri in 0..HEIHGT {
            for ci in 0..WIDTH { 
                print!("{}", cpu.framebuffer[ri as usize][ci as usize]);
            }
            println!("");
        }
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





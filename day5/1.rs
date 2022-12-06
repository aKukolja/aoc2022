use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Stacks {
    collect: bool,
    lines_stack: Vec<String>,
    storage: Vec<Vec<char>>,
}

impl Stacks {

    pub fn new() -> Self {
        Self {
            collect: true,
            lines_stack: Vec::new(),
            storage: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) -> (){
        let mut si = 0;
        let mut i = 1;
        while i < line.len() {
            if let Some(package) = line.chars().nth(i) {
                if self.storage.len() <= si {
                    self.storage.push(Vec::new());
                }
                if package.is_alphabetic() {
                    self.storage.get_mut(si).unwrap().push(package);
                }
            } else {
                break;
            }
            i += 4;
            si += 1;
        }
    }

    fn perform(&mut self, l: String) {
        let parts: Vec<&str> = l.split(" ").collect();
        let mut count = parts.get(1).unwrap().parse::<usize>().unwrap();
        let from = parts.get(3).unwrap().parse::<usize>().unwrap();
        let to = parts.get(5).unwrap().parse::<usize>().unwrap();
        while count > 0 {
            count -= 1;
            let ff = self.storage.get_mut(from-1).unwrap();
            let taken = ff.pop().unwrap();
            let tt = self.storage.get_mut(to-1).unwrap();
            tt.push(taken);
        }
    }

    pub fn get_result(&self) -> String {
        self.storage.iter().filter_map(|s| s.last()).collect::<String>()
    }

    pub fn handle_line(&mut self, l: String) -> () {
        if l.len() == 0 {
        } else if !self.collect {
            self.perform(l);
        } else if l.chars().nth(1).unwrap().is_numeric() {
            self.collect = false;
            // restack
            while self.lines_stack.len() > 0 {
                let candidate = self.lines_stack.pop().unwrap();
                self.add_line(&candidate)
                
            }
        } else {
            self.lines_stack.push(l);
        }
    }

}


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut stacks = Stacks::new();
        for line in lines {
            if let Ok(as_str) = line {
                stacks.handle_line(as_str);
            }
        }
        println!("{}", stacks.get_result());
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





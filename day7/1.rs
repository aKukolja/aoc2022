use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap};
use std::cell::RefCell;
use std::rc::Rc;

type DirHandle = Rc<RefCell<Directory>>;

#[derive(Default)]
struct Directory {
    size: u64,
    children: HashMap<String, DirHandle>,
    parent: Option<DirHandle>,
}

impl Directory {
    pub fn add_file(&mut self, _name: String, size: u64) -> () {
        self.size += size;
    }
}

enum Line {
    Cd {destination: String},
    Ls,
    Dir{name: String},
    File{size: u64, name: String},
}

fn parse_line(line: String) -> Line {
    if line.starts_with("$ cd ") {
        return Line::Cd {destination: line[5..].to_string()};
    } else if line.starts_with("$ ls") {
        return Line::Ls {};
    } else if line.starts_with("dir ") {
        return Line::Dir {name: line[4..].to_string()};
    }
    let parts: Vec<&str> = line.split(" ").collect();
    match &parts[..] {
        [val, name] => {
            if let Ok(bbs) = val.parse::<u64>() {
                return Line::File { name: name.to_string(), size: bbs};
            }
        },
        _ => todo!()
    }
    panic!("Invalid line {}", line);
}

fn iter_tree(root: &DirHandle) -> (u64, u64) {
    let mut sol_sum = 0;  // sum of all directories <= 100000
    let mut curr_sum = root.borrow().size;  // start with sum of files
    for (_, child) in &root.borrow_mut().children {
        let (child_sum, child_solution) = iter_tree(child);
        curr_sum += child_sum;  // add child directory to our size
        sol_sum += child_solution;
    }
    root.borrow_mut().size = curr_sum;
    if curr_sum <= 100000 {
        sol_sum += curr_sum;
    }
    return (curr_sum, sol_sum);
}

fn main() {
    let root: DirHandle = Rc::new(RefCell::new(Directory::default()));
    let mut cd: DirHandle = root.clone();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(as_str) = line {
                match parse_line(as_str) {
                    Line::Dir{name} => {
                        let entry = cd.borrow_mut().children.entry(name).or_default().clone();
                        entry.borrow_mut().parent = Some(cd.clone());
                    },
                    Line::File{size, name} => {
                        cd.borrow_mut().add_file(name, size);
                    }
                    Line::Cd{destination} => {
                        match &destination[..] {
                            "cd" => {
                                cd = root.clone();
                            }
                            "/" => {
                                cd = root.clone();
                            }
                            ".." => {
                                let tmp = cd.borrow().parent.clone().unwrap();
                                cd = tmp;
                            }
                            _ => {
                                let tmp = cd.borrow_mut().children.entry(destination).or_default().clone();
                                cd = tmp;
                            }
                        }
                    },
                    Line::Ls => (),
                }
            }
        }
    }
    let (_, result) = iter_tree(&root);
    println!("{}", result);
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





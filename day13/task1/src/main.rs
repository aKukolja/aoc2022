use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Task {
    Element(i32),
    ElementList(Vec<Task>)
}


fn compare(left: &Task, right: &Task) -> Ordering {
    //println!("compare {:?} < {:?}", left, right);
    match (left, right) {
        (Task::Element(vl), Task::Element(vr)) => {
            //println!(" = {}", vl < vr);
            if vl == vr {
                return Ordering::Equal;
            } else if vl < vr {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        (Task::ElementList(ll), Task::ElementList(lr)) => {
            for (lv, rv) in ll.iter().zip(lr.iter()) {
                match compare(lv, rv) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            if ll.len() < lr.len() {
                return Ordering::Less;
            }
            if ll.len() == lr.len() {
                return Ordering::Equal;
            }
            return Ordering::Greater;
        }
        (Task::Element(vl), Task::ElementList(_lr)) => {
            let ll = Task::ElementList(vec![Task::Element(*vl)]);
            let retval = compare(&ll, right);
            //println!(" = {}", retval);
            return retval;
        }
        (Task::ElementList(_ll), Task::Element(vr)) => {
            let lr = Task::ElementList(vec![Task::Element(*vr)]);
            let retval = compare(left, &lr);
            //println!(" = {}", retval);
            return retval;
        }
    }
}

#[derive(Default)]
struct Pair {
    l1: Option<Task>,
    l2: Option<Task>
}

impl Pair {
    fn consume(&mut self) -> bool {
        if self.l1.is_none() {
            panic!("Missing first");
        }
        if self.l2.is_none() {
            panic!("Missing first");
        }
        let retval = compare(&self.l1.as_ref().unwrap(), &self.l2.as_ref().unwrap());
        self.l1 = None;
        self.l2 = None;
        match retval {
            Ordering::Less => true,
            _ => false
        }
    }
    fn add_list(&mut self, tl: Task) -> () {
        if self.l1.is_none() {
            self.l1 = Some(tl);
            return;
        }
        if self.l2.is_none() {
            self.l2 = Some(tl);
            return;
        }
        panic!("Already have two Task");
    }
    fn full(&self) -> bool {
        return self.l1.is_some() && self.l2.is_some();
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut p = Pair::default();
        let mut sum = 0;
        let mut i = 0;
        for line in lines {
            if let Ok(as_str) = line {
                if as_str.len() != 0 {
                    let l: Task = serde_json::from_str(&as_str).unwrap();
                    p.add_list(l);
                    if p.full() {
                        i += 1;
                        if p.consume() {
                            println!("Equal index {}", i);
                            sum += i;
                        }
                    }
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



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

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        match compare(self, other) {
            Ordering::Equal => true,
            _ => false
        }
    }
}

impl Eq for Task {}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let sentinels: Vec<Task> = vec![
            serde_json::from_str("[[2]]").unwrap(),
            serde_json::from_str("[[6]]").unwrap()
        ];
        let mut e: Vec<Task> = Vec::new();
        e.push(sentinels[0].clone());
        e.push(sentinels[1].clone());
        for line in lines {
            if let Ok(as_str) = line {
                if as_str.len() != 0 {
                    let l: Task = serde_json::from_str(&as_str).unwrap();
                    e.push(l);
                }
            }
        }
        e.sort();
        let retval = sentinels.iter().map(|s| e.binary_search(s).unwrap() + 1).product::<usize>();
        println!("{}", retval);
    }
}


fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

lazy_static! {
    static ref MOPS: Regex = Regex::new(r"^(\w+): (\w+) ([+-/*]) (\w+)$").unwrap();
    static ref CONSTS: Regex = Regex::new(r"^(\w+): (-?\d+)$").unwrap();
}

const ADD: fn(&i64, &i64) -> i64 = |a, b| {a + b};
const SUB: fn(&i64, &i64) -> i64 = |a, b| {a - b};
const MUL: fn(&i64, &i64) -> i64 = |a, b| {a * b};
const DIV: fn(&i64, &i64) -> i64 = |a, b| {a / b};

type MonkeyBusiness = (fn (&i64, &i64) -> i64, String, String);

#[derive(Default)]
struct Monkeys {
    known: HashMap<String, i64>,
    unknown: HashMap<String, MonkeyBusiness>,
}

impl Monkeys {
    fn add_line(&mut self, s: String) {
        if let Some(ccs) = MOPS.captures(&s) {
            let m_id = ccs.get(1).unwrap().as_str().to_string();
            let a = ccs.get(2).unwrap().as_str().to_string();
            let op = ccs.get(3).unwrap().as_str();
            let b = ccs.get(4).unwrap().as_str().to_string();
            let mb = match op {
                "+" => (ADD, a, b),
                "-" => (SUB, a, b),
                "*" => (MUL, a, b),
                "/" => (DIV, a, b),
                _ => panic!("Invalid input {}", s),
            };
            self.unknown.insert(m_id, mb);
        } else if let Some(ccs) = CONSTS.captures(&s) {
            let m_id = ccs.get(1).unwrap().as_str().to_string();
            let bp_id = ccs.get(2).unwrap().as_str().parse::<i64>().unwrap();
            self.known.insert(m_id, bp_id);
        } else {
            panic!("Incorrect input {}", s);
        }
    }
    fn solve(&mut self) -> i64 {
        while self.unknown.len() > 0 {
            self.unknown.retain(|key, (op, a, b)| {
                if let (Some(av), Some(bv)) = (self.known.get(a), self.known.get(b)) {
                    self.known.insert(key.clone(), op(av, bv));
                    false
                } else {
                    true
                }
            });
        }
        return *self.known.get("root").unwrap();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut mms = Monkeys::default();

    for line in reader.lines() {
        let ll = line?;
        mms.add_line(ll);
    }

    println!("{}", mms.solve());

    return Ok(());
}

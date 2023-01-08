use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

lazy_static! {
    static ref MOPS: Regex = Regex::new(r"^(\w+): (\w+) ([+-/*]) (\w+)$").unwrap();
    static ref CONSTS: Regex = Regex::new(r"^(\w+): (-?\d+)$").unwrap();
}

const HUMAN: &str = "humn";
const ROOT: &str = "root";

#[derive(Clone)]
enum Operation {
    Add,
    Mul,
    Sub,
    Div,
}

impl Operation {
    fn do_op(&self, a: i64, b: i64) -> i64 {
        match *self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
        }
    }
    fn do_reverse(&self, a: i64, b: i64) -> i64 {
        match *self {
            Operation::Add => a - b,
            Operation::Sub => a + b,
            Operation::Mul => a / b,
            Operation::Div => a * b,
        }
    }
}

// human is monkey, lol
// only one human, is leaf node?
// only look at subtree of the human node
#[derive(Clone)]
enum Monkey {
    Constant(i64),
    Operator(Operation, String, String),
}

#[derive(Default)]
struct Monkeys {
    known: HashMap<String, i64>,
    tree: HashMap<String, Monkey>,
}

impl Monkeys {
    fn add_line(&mut self, s: String) {
        if let Some(ccs) = MOPS.captures(&s) {
            let m_id = ccs.get(1).unwrap().as_str().to_string();
            let a = ccs.get(2).unwrap().as_str().to_string();
            let op = ccs.get(3).unwrap().as_str();
            let b = ccs.get(4).unwrap().as_str().to_string();
            let mb = match op {
                "+" => Monkey::Operator(Operation::Add, a, b),
                "-" => Monkey::Operator(Operation::Sub, a, b),
                "*" => Monkey::Operator(Operation::Mul, a, b),
                "/" => Monkey::Operator(Operation::Div, a, b),
                _ => panic!("Invalid input {}", s),
            };
            self.tree.insert(m_id, mb);
        } else if let Some(ccs) = CONSTS.captures(&s) {
            let m_id = ccs.get(1).unwrap().as_str().to_string();
            let bp_id = ccs.get(2).unwrap().as_str().parse::<i64>().unwrap();
            self.tree.insert(m_id.clone(), Monkey::Constant(bp_id));
            self.known.insert(m_id, bp_id);
        } else {
            panic!("Incorrect input {}", s);
        }
    }

    fn calc_value(&mut self, curr: &String) -> (i64, bool) {
        match self.get_copy(curr) {
            Monkey::Constant(cval) => (cval, curr == &HUMAN),
            Monkey::Operator(op, left, right) => {
                let (lval, lh) = self.calc_value(&left);
                let (rval, rh) = self.calc_value(&right);
                let retval = op.do_op(lval, rval);
                self.known.insert(curr.clone(), retval); // speed up by not cloning?
                (retval, lh || rh)
            }
        }
    }

    fn bfs(&self, search_side: &String, expected: i64) -> i64 {
        let mut q: VecDeque<(&String, i64)> = VecDeque::new();
        q.push_back((search_side, expected));
        while let Some((curr, ev)) = q.pop_front() {
            // if curr == HUMAN, return value
            // we are expected_value == left op right
            // assume human is in left subtree -> expected_value = unknown op right
            if curr == &HUMAN {
                return ev;
            }
            match self.tree.get(curr).unwrap() {
                Monkey::Constant(_) => {
                    continue;
                },
                Monkey::Operator(op, left, right) => {
                    // do human on left side
                    q.push_back((left, op.do_reverse(ev, *self.known.get(right).unwrap())));
                    // do human on right side
                    let lv = self.known.get(left).unwrap();
                    let vv = match op {
                        Operation::Add | Operation::Mul => op.do_reverse(ev, *lv),
                        Operation::Div => {
                            if ev == 0 {
                                lv+1  // this is real dirty lol, doesnt work in general case
                            } else {
                                op.do_op(*lv, ev)
                            }
                        },
                        _ => op.do_op(*lv, ev),
                    };
                    q.push_back((right, vv));
                }
            }
        }
        panic!("No solution");
    }

    fn get_copy(&self, k: &str) -> Monkey {
        self.tree.get(k).unwrap().clone()  // TODO: stop cloning to statisfy borrow checker
    }

    fn solve(&mut self) -> i64 {
        if let Monkey::Operator(_, left, right) = self.get_copy(ROOT) {
            let (left_value, human_left) = self.calc_value(&left);
            let (right_value, human_right) = self.calc_value(&right);
            if !(human_left ^ human_right) {
                panic!("Multiple humans in tree");
            }
            let expected_value = if human_left { right_value } else { left_value };
            let search_side = if human_left { left } else { right };

            return self.bfs(&search_side, expected_value);
        } else {
            panic!("Root node must be Operator type");
        }
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

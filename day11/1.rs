use std::collections::VecDeque;


// TODO: implement monkey parsing
struct Monkey {
    items: VecDeque<i32>,
    thinking: fn(i32) -> i32,
    throw_test: fn(i32) -> bool,
    throw_true: usize,
    throw_false: usize
}

fn new_items(me: &mut Monkey) -> (VecDeque<i32>, VecDeque<i32>) { 
    let mut t = VecDeque::new();
    let mut f = VecDeque::new();
    while me.items.len() > 0 {
        let item = me.items.pop_front().unwrap();
        let new_item = (me.thinking)(item) / 3;
        if (me.throw_test)(new_item) {
            t.push_back(new_item);
        } else {
            f.push_back(new_item);
        }
    }
    return (t, f);
}

// TODO: monkeys.split_at_mut is a terrible workaround, smh
fn round(monkeys: &mut [Monkey], counters: &mut [i32]) -> () {
    for i in 0..monkeys.len() {
        let me = &mut monkeys[i];
        let (ti, fi) = (me.throw_true, me.throw_false);
        let (mut t, mut f) = new_items(me);
        while t.len() > 0 {
            counters[i] += 1;
            monkeys[ti].items.push_back(t.pop_front().unwrap());
        }
        while f.len() > 0 {
            counters[i] += 1;
            monkeys[fi].items.push_back(f.pop_front().unwrap());
        }
    }
}

fn main() {
    let mut monkeys: [Monkey; 8] = [
        Monkey {  // 0
            items: VecDeque::from(vec![76, 88, 96, 97, 58, 61, 67]),
            thinking: |old|  {old * 19},
            throw_test: |val| {val % 3 == 0},
            throw_true: 2,
            throw_false: 3
        },
        Monkey {  // 1
            items: VecDeque::from(vec![93, 71, 79, 83, 69, 70, 94, 98]),
            thinking: |old|  {old + 8},
            throw_test: |val| {val % 11 == 0},
            throw_true: 5,
            throw_false: 6
        },
        Monkey {  // 2
            items: VecDeque::from(vec![50, 74, 67, 92, 61, 76]),
            thinking: |old|  {old * 13},
            throw_test: |val| {val % 19 == 0},
            throw_true: 3,
            throw_false: 1
        },
        Monkey {  // 3
            items: VecDeque::from(vec![76, 92]),
            thinking: |old|  {old + 6},
            throw_test: |val| {val % 5 == 0},
            throw_true: 1,
            throw_false: 6
        },
        Monkey {  // 4
            items: VecDeque::from(vec![74, 94, 55, 87, 62]),
            thinking: |old|  {old + 5},
            throw_test: |val| {val % 2 == 0},
            throw_true: 2,
            throw_false: 0
        },
        Monkey {  // 5
            items: VecDeque::from(vec![59, 62, 53, 62]),
            thinking: |old|  {old * old},
            throw_test: |val| {val % 7 == 0},
            throw_true: 4,
            throw_false: 7
        },
        Monkey {  // 6
            items: VecDeque::from(vec![62]),
            thinking: |old|  {old + 2},
            throw_test: |val| {val % 17 == 0},
            throw_true: 5,
            throw_false: 7
        },
        Monkey {  // 7
            items: VecDeque::from(vec![85, 54, 53]),
            thinking: |old|  {old + 3},
            throw_test: |val| {val % 13 == 0},
            throw_true: 4,
            throw_false: 0
        },
    ];
    let mut counters = [0; 8];
    for _ in 0..20 {
        round(&mut monkeys, &mut counters);
    }
    counters.sort_by(|a, b| b.cmp(a));
    println!("{}", counters[0] * counters[1]);
} 

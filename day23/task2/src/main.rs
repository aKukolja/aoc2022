use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// x, y; x grows left to right, y grows downwards
type Coord = (i32, i32);

#[derive(Default)]
struct World {
    elfs: HashSet<Coord>,
}

const DIRECTION_DIFFS: [[Coord; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // up
    [(-1, 1), (0, 1), (1, 1)],    // down
    [(-1, -1), (-1, 0), (-1, 1)], // left
    [(1, -1), (1, 0), (1, 1)],    // right
];

impl World {
    fn add_line(&mut self, line: &str, y: i32) -> () {
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    self.elfs.insert((i as i32, y));
                }
                _ => (),
            }
        }
    }

    fn solve(&mut self) -> usize {
        for offset in 0.. {
            let _offset = offset % DIRECTION_DIFFS.len();
            let mut propositions: HashMap<Coord, Vec<Coord>> = HashMap::new();
            for elf in self.elfs.iter() {
                if DIRECTION_DIFFS
                    .iter()
                    .map(|steps| steps.iter().map(|step| (elf.0 + step.0, elf.1 + step.1)))
                    .flatten()
                    .all(|kernel| !self.elfs.contains(&kernel))
                {
                    continue;
                }
                for steps in DIRECTION_DIFFS
                    .iter()
                    .cycle()
                    .skip(_offset)
                    .take(DIRECTION_DIFFS.len())
                {
                    if steps
                        .iter()
                        .map(|step| (elf.0 + step.0, elf.1 + step.1))
                        .all(|next_elf| !self.elfs.contains(&next_elf))
                    {
                        let proposed_position = (elf.0 + steps[1].0, elf.1 + steps[1].1);
                        propositions.entry(proposed_position).or_insert(Vec::new()).push(*elf);
                        break;
                    }
                }
            }

            if propositions.len() == 0 {
                return offset + 1;
            }

            let next_elfs: Vec<Coord> = self
                .elfs
                .iter()
                .map(|elf| {
                    let next_pos = DIRECTION_DIFFS
                        .iter()
                        .cycle()
                        .skip(_offset)
                        .take(DIRECTION_DIFFS.len())
                        .map(|steps| (elf.0 + steps[1].0, elf.1 + steps[1].1))
                        .find_map(|next_elf| {
                            propositions
                                .get(&next_elf)
                                .filter(|candidates| candidates.len() == 1 && candidates[0] == *elf)
                                .map(|_| next_elf)
                        })
                        .or(Some(*elf));
                    next_pos
                })
                .flatten()
                .collect();

            self.elfs = next_elfs.into_iter().collect();
        }
        panic!("What?!");
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../example.txt")?;
    let reader = BufReader::new(file);

    let mut w = World::default();

    for (y, line) in reader.lines().enumerate() {
        let ll = line?;
        w.add_line(&ll, y as i32);
    }

    println!("{}", w.solve());

    return Ok(());
}

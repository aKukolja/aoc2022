use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// x, y; x grows left to right, y grows downwards
#[derive(Eq, PartialEq, Debug)]
struct Pixel {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
type WorldState = Vec<Vec<Pixel>>;

fn mutate_world(world: &WorldState) -> WorldState {
    // scan up -> down, advance up arrows
    // TODO: move bottom row
    let mut retval: WorldState = Vec::new();

    // move up arrows and create new world
    // TODO: DO NOT MUTATE WORLD
    let sentinel_winds: Vec<bool> = world[0].iter().map(|p| p.up).collect();
    for y in 1..(world.len()) {
        retval.push(
            world[y]
                .iter()
                .map(|p| Pixel {
                    up: p.up,
                    down: false,
                    left: false,
                    right: false,
                })
                .collect(),
        );
    }
    // add up arrows from top row to bottom row
    retval.push(
        sentinel_winds
            .iter()
            .map(|b| Pixel {
                up: *b,
                down: false,
                left: false,
                right: false,
            })
            .collect(),
    );

    // scan down -> up
    let sentinel_winds: Vec<bool> = world.last().unwrap().iter().map(|p| p.down).collect();
    for y in (0..(world.len() - 1)).rev() {
        for (low, top) in retval[y + 1].iter_mut().zip(world[y].iter()) {
            low.down = top.down;
        }
    }
    for (rr, ss) in retval[0].iter_mut().zip(sentinel_winds) {
        rr.down = ss;
    }

    // scan left -> right
    let sentinel_winds: Vec<bool> = (0..world.len()).map(|y| world[y][0].left).collect();
    for y in 0..world.len() {
        for x in 0..(world[y].len() - 1) {
            retval[y][x].left = world[y][x + 1].left;
        }
        retval[y].last_mut().unwrap().left = sentinel_winds[y];
    }

    let sentinel_winds: Vec<bool> = (0..world.len())
        .map(|y| world[y].last().unwrap().right)
        .collect();
    for y in 0..world.len() {
        for x in (0..(world[y].len() - 1)).rev() {
            retval[y][x + 1].right = world[y][x].right;
        }
        retval[y][0].right = sentinel_winds[y];
    }

    retval
}

#[derive(Default)]
struct World {
    world_states: Vec<WorldState>,
}

fn print_pixel(p: &Pixel) -> String {
    let mut c = 0;
    let mut retval = "";
    if p.up {
        c += 1;
        retval = "^";
    }
    if p.down {
        c += 1;
        retval = "v";
    }
    if p.left {
        c += 1;
        retval = "<";
    }
    if p.right {
        c += 1;
        retval = ">";
    }
    if c == 0 {
        return ".".to_string();
    }
    if c == 1 {
        return retval.to_string();
    }
    return format!("{}", c);
}

fn print_world(w: &WorldState) {
    for wr in w {
        for c in wr {
            print!("{}", print_pixel(c));
        }
        println!("");
    }
    println!("");
}

impl World {
    fn add_line(&mut self, line: &str) -> () {
        if self.world_states.len() == 0 {
            self.world_states.push(Vec::new());
        }
        let world_row: Vec<Pixel> = line
            .chars()
            .map(|c| match c {
                '.' => Some(Pixel {
                    up: false,
                    down: false,
                    left: false,
                    right: false,
                }),
                '#' => None,
                '^' => Some(Pixel {
                    up: true,
                    down: false,
                    left: false,
                    right: false,
                }),
                '<' => Some(Pixel {
                    up: false,
                    down: false,
                    left: true,
                    right: false,
                }),
                '>' => Some(Pixel {
                    up: false,
                    down: false,
                    left: false,
                    right: true,
                }),
                'v' => Some(Pixel {
                    up: false,
                    down: true,
                    left: false,
                    right: false,
                }),
                _ => panic!("Unexpected"),
            })
            .flatten()
            .collect();
        self.world_states[0].push(world_row);
    }

    fn discover_states(&mut self) -> () {
        loop {
            let next_state = mutate_world(self.world_states.last().unwrap());
            if compare_worlds(&next_state, &self.world_states[0]) {
                break;
            }
            // print_world(&next_state);
            self.world_states.push(next_state);
        }
    }

    fn taken(&self, nt: usize, y: i32, x: i32) -> bool {
        let p = &self.world_states[nt][y as usize][x as usize];
        p.up || p.down || p.left || p.right
    }

    fn solve(&mut self) -> usize {
        self.discover_states();
        let start = (-1, 0);
        let end = (
            self.world_states[0].len(),
            self.world_states[0][0].len() - 1,
        );

        // DFS
        // state: coordinate, steps (step is time), normalized steps (aka world time)
        let mut stack: Vec<((i32, i32), usize, usize)> = Vec::new();
        stack.push((start, 0, 0));

        let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();

        let mut solution: Option<usize> = None;

        while stack.len() > 0 {
            let ((y, x), steps, normalized) = stack.pop().unwrap();

            if visited.contains(&((y, x), normalized)) {
                continue;
            }
            visited.insert(((y, x), normalized));

            if let Some(ss) = solution {
                if steps >= ss {
                    continue;
                }
            }

            let next_step = steps + 1;
            let next_normalized = (steps + 1) % self.world_states.len();

            for (ny, nx) in [(y - 1, x), (y, x - 1), (y, x), (y + 1, x), (y, x + 1)] {
                if (ny, nx) == (end.0 as i32, end.1 as i32) {
                    if let Some(old) = solution {
                        if old > steps + 1 {
                            solution = Some(steps + 1);
                        }
                    } else {
                        solution = Some(steps + 1)
                    }
                    break;
                }
                if (ny, nx) != start && ny < 0 || nx < 0 {
                    continue;
                }
                if (ny, nx) != start && ny as usize >= self.world_states[normalized].len() {
                    continue;
                }
                if (ny, nx) != start
                    && nx as usize >= self.world_states[normalized][ny as usize].len()
                {
                    continue;
                }

                if (ny, nx) != start && self.taken(next_normalized, ny, nx) {
                    continue;
                }

                stack.push(((ny, nx), next_step, next_normalized))
            }
        }

        if let Some(ss) = solution {
            return ss;
        }
        panic!("Invalid map");
    }
}

fn compare_worlds(a: &WorldState, b: &WorldState) -> bool {
    for (ra, rb) in a.iter().zip(b.iter()) {
        for (ca, cb) in ra.iter().zip(rb.iter()) {
            if ca != cb {
                return false;
            }
        }
    }
    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut w = World::default();

    for (y, line) in reader.lines().enumerate() {
        if y == 0 {
            continue;
        }
        let ll = line?;
        w.add_line(&ll);
    }
    w.world_states[0].pop();

    println!("{}", w.solve());

    return Ok(());
}

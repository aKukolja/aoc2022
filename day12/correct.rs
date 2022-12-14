use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Element {
    Start,
    End,
    Value(i8)
}

const TERMINATOR: i8 = 'z' as i8 - 'a' as i8;

impl Element {
    fn value(&self) -> i8 {
        match *self {
            Element::Start => 0,
            Element::End => TERMINATOR,
            Element::Value(v) => v,
        }
    }
}

type Coord = (usize, usize);


#[derive(Default)]
struct Mountains {
    si: Coord,
    ei: Coord,
    scan: Vec<Vec<Element>>
}

impl Mountains {

    fn add_row(&mut self, row: Vec<Element>) {
        let height_index = self.scan.len();
        for (width_index, e) in row.iter().enumerate() {
            match e {
                Element::Start => {
                    self.ei = (height_index, width_index);
                }
                Element::End => {
                    self.si = (height_index, width_index);
                }
                _ => ()
            }
        }
        self.scan.push(row);
    }

    fn bfs(&self) -> usize {
        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let height = self.scan.len();
        let width = self.scan[0].len();
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut q: VecDeque<(Coord, usize)> = VecDeque::new();

        q.push_back((self.si, 0));
        while q.len() > 0 {
            let (curr_i, path_len) = q.pop_front().unwrap();
            if visited.contains(&curr_i) {
                continue;
            }
            visited.insert(curr_i);

            let e_val = &self.scan[curr_i.0][curr_i.1];
            match e_val {
                Element::Start => {
                    return path_len;
                }
                _ => {
                    let curr_height = e_val.value();
                    if curr_height == 0 {
                        return path_len;
                    }
                    for direction in directions {
                        let next_width_i = direction.1 + curr_i.1 as i32;
                        let next_height_i  = direction.0 + curr_i.0 as i32;
                        if next_height_i >= height as i32 || next_height_i < 0 || next_width_i >= width as i32 || next_width_i < 0 {
                            continue;
                        }
                        let next_width = next_width_i as usize;
                        let next_height = next_height_i as usize;
                        let next_element = &self.scan[next_height][next_width];
                        match next_element {
                            Element::End => {
                                continue;
                            }
                            Element::Start => {
                                q.push_back(((next_height, next_width), path_len + 1));
                            }
                            Element::Value(vvv) => {
                                if curr_height - vvv <= 1 {
                                    q.push_back(((next_height, next_width), path_len + 1));
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("No solution found");
        return 0;
    }

}

fn parse_line(line: &str) -> Vec<Element> {
    return line.chars().map(|c| match c {
        'E' => Element::End,
        'S' => Element::Start,
        _ => {
            Element::Value(c as i8 - 'a' as i8)
        }
    }).collect();
}


fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut m = Mountains::default();
        for line in lines {
            if let Ok(as_str) = line {
                m.add_row(parse_line(&as_str))
            }
        }
        println!("{}", m.bfs())
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





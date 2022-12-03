use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calculate(s : &str) -> [i32; 26*2] {
    let mut count = [0; 26*2];
    for c in s.chars() {
        let ind = if c.is_lowercase() {
            c as usize - 'a' as usize
        } else {
            c as usize - 'A' as usize + 26
        };
        count[ind] += 1;
    }
    return count;
}

fn main() {
    // mut lines feels wrong, why ever take mutable here?
    if let Ok(mut lines) = read_lines("./input.txt") {
        let mut sum = 0;
        loop {
            let s1 = lines.next();
            let s2 = lines.next();
            let s3 = lines.next();
            if let (Some(Ok(ss1)), Some(Ok(ss2)), Some(Ok(ss3))) = (s1, s2, s3) {
                let c1 = calculate(&ss1);
                let c2 = calculate(&ss2);
                let c3 = calculate(&ss3);
                for i in 0..52 {
                    if c1[i] >= 1 && c2[i] >= 1 && c3[i] >= 1 {
                        sum += i + 1;
                    }
                }
            } else {
                break;
            }
            
        }
        println!("{}", sum);
    }
} 

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
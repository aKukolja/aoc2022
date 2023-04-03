use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn from_snafu(s: &str) -> i64 {
    let mut retval = 0;
    let mut pos = 1;

    for c in s.chars().rev() {
        let factor = match c {
            '1' => 1,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character")
        };
        retval += factor * pos;

        pos *= 5;
    }

    return retval;
}


fn to_snafu(i: i64) -> String {
    println!("{}", i);
    let ddd: usize = 20;
    let mut val = i;
    let mut factor = 5_i64.pow((ddd-1) as u32);

    let mut digits: Vec<i8> = vec![0; ddd];
    let mut ii = ddd;

    while ii > 0 {
        // println!("{}", val);
        digits[ii-1] = (val / factor) as i8;
        ii -= 1;
        val = val % factor;
        factor /= 5;
    }
    println!("{:?}", digits);

    for jj in 0..digits.len()-1 {
        move_digit(&mut digits, jj);
    }

    /* 
    ii = ddd-1;
    while true {
        if move_digit(&mut digits, ii) {
            for jj in ii..ddd-1 {
                if !move_digit(&mut digits, jj) {
                    break;
                }
            }
        }
        if ii == 0 {
            break;
        }
        ii -= 1;
    }
    */

    println!("{:?}", digits);
    for d in digits.iter().rev() {
        let c = match d {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => panic!("")
        };
        print!("{}", c);
    }

    let mut retval = "".to_string();

    return retval;
}

fn move_digit(digits: &mut Vec<i8>, i: usize) -> bool{
    if digits[i] == 3 {
        digits[i+1] += 1;
        digits[i] = -2;
        return true;
    }else if digits[i] == 4 {
        digits[i+1] += 1;
        digits[i] = -1;
        return true;
    } else if digits[i] == 5 {
        digits[i+1] += 1;
        digits[i] = 0;
    }
    return false;
}


fn main() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += from_snafu(&line?);
    }

    // println!("{}", from_snafu(&to_snafu(sum)));

    println!("{}", to_snafu(sum));

    return Ok(());
}

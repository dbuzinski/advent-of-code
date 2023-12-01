use std::io::{BufRead, BufReader, Lines, Result};
use std::fs::File;

fn main() {
    let filepath = "data/01.txt";

    // Part 1
    let lines = read_lines(filepath).unwrap();
    let mut s = 0;
    for line in lines {
        if let Ok(txt) = line {
            s += get_line_calibration(&txt);
        }
    }
    println!("{}", s);
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_line_calibration(line: &str) -> u32 {
    let v: Vec<&str> = line.matches(char::is_numeric).collect();
    let first = u32::from_str_radix(v.first().unwrap(), 10).unwrap();
    let last = u32::from_str_radix(v.last().unwrap(), 10).unwrap();
    10 * first + last
}

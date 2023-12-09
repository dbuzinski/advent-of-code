use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;

fn main() {
    let filepath = "data/01.txt";

    // Part 2
    let lines = read_lines(filepath).unwrap();
    let mut s = 0;
    for line in lines {
        if let Ok(txt) = line {
            s += get_line_calibration(&txt).unwrap();
        }
    }
    println!("Day 1 part 2: {}", s);
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_line_calibration(line: &str) -> Result<u32, String> {
    let first = find_first_num(line).unwrap();
    let last = find_last_num(line).unwrap();
    Ok(first*10+last)
}

fn find_first_num(s: &str) -> Result<u32, &str> {
    match s {
        t if t.starts_with("zero") => Ok(0),
        t if t.starts_with("0") => Ok(0),
        t if t.starts_with("one") => Ok(1),
        t if t.starts_with("1") => Ok(1),
        t if t.starts_with("two") => Ok(2),
        t if t.starts_with("2") => Ok(2),
        t if t.starts_with("three") => Ok(3),
        t if t.starts_with("3") => Ok(3),
        t if t.starts_with("four") => Ok(4),
        t if t.starts_with("4") => Ok(4),
        t if t.starts_with("five") => Ok(5),
        t if t.starts_with("5") => Ok(5),
        t if t.starts_with("six") => Ok(6),
        t if t.starts_with("6") => Ok(6),
        t if t.starts_with("seven") => Ok(7),
        t if t.starts_with("7") => Ok(7),
        t if t.starts_with("eight") => Ok(8),
        t if t.starts_with("8") => Ok(8),
        t if t.starts_with("nine") => Ok(9),
        t if t.starts_with("9") => Ok(9),
        t if !t.get(1..).is_none() => find_first_num(t.get(1..).unwrap()),
        _ => Err("Did not find numeric value in line"),
    }
}

fn find_last_num(s: &str) -> Result<u32, &str> {
    match s {
        t if t.ends_with("zero") => Ok(0),
        t if t.ends_with("0") => Ok(0),
        t if t.ends_with("one") => Ok(1),
        t if t.ends_with("1") => Ok(1),
        t if t.ends_with("two") => Ok(2),
        t if t.ends_with("2") => Ok(2),
        t if t.ends_with("three") => Ok(3),
        t if t.ends_with("3") => Ok(3),
        t if t.ends_with("four") => Ok(4),
        t if t.ends_with("4") => Ok(4),
        t if t.ends_with("five") => Ok(5),
        t if t.ends_with("5") => Ok(5),
        t if t.ends_with("six") => Ok(6),
        t if t.ends_with("6") => Ok(6),
        t if t.ends_with("seven") => Ok(7),
        t if t.ends_with("7") => Ok(7),
        t if t.ends_with("eight") => Ok(8),
        t if t.ends_with("8") => Ok(8),
        t if t.ends_with("nine") => Ok(9),
        t if t.ends_with("9") => Ok(9),
        t if !t.get(..t.len()-1).is_none() => find_last_num(t.get(..t.len()-1).unwrap()),
        _ => Err("Did not find numeric value in line"),
    }
}

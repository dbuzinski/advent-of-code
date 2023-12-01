use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;

fn main() {
    let filepath = "data/01.txt";

    // Part 1
    let lines = read_lines(filepath).unwrap();
    let mut s = 0;
    for line in lines {
        if let Ok(txt) = line {
            s += get_line_calibration(&txt).unwrap();
        }
    }
    println!("Part 1: {}", s);
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_line_calibration(line: &str) -> Result<u32, String> {
    let v: Vec<&str> = line.matches(char::is_numeric).collect();
    let err = format!("{}: {}", "Did not find numeric value in line", line);
    let first = v.first().ok_or(&err)?;
    let last = v.last().ok_or(&err)?;
    let mut calibration_string = String::from(*first);
    calibration_string.push_str(*last);
    let calibration: u32 = calibration_string.parse().unwrap();
    Ok(calibration)
}

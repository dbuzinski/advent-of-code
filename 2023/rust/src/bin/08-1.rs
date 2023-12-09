use std::collections::HashMap;
use std::io::{self, BufRead ,BufReader, Lines};
use std::fs::File;

fn main() {
    let filename = "data/08.txt";
    let mut lines = read_lines(filename).unwrap();
    let lr_line = lines.next().unwrap().unwrap();
    lines.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        if let Ok(txt) = line {
            let (k, v) = get_kv_pair(&txt);
            map.insert(k, v);
        }
    }
    let mut loc = String::from("AAA");
    let mut lr_iter = lr_line.chars();
    let mut num_steps = 0;
    while loc != String::from("ZZZ") {
        match lr_iter.next() {
            Some('L') => {
                num_steps = num_steps + 1;
                loc = map.get(&loc).unwrap().0.clone();
            },
            Some('R') => {
                num_steps = num_steps + 1;
                loc = map.get(&loc).unwrap().1.clone();
            },
            _ => {
                lr_iter = lr_line.chars();
            },
        }
    }
    println!("Day 8 part 1: {}", num_steps);
}

fn get_kv_pair(line: &str) -> (String, (String, String)){
    let key = String::from(line.get(..3).unwrap());
    let left = String::from(line.get(7..10).unwrap());
    let right = String::from(line.get(12..15).unwrap());
    (key, (left, right))
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}


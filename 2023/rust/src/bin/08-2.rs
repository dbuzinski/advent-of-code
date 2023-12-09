use std::collections::HashMap;
use std::io::{self, BufRead ,BufReader, Lines};
use std::fs::File;
use num::integer::lcm;

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
    let start = get_start(&map);
    let mut loc: String;
    let mut lr_iter;
    let mut num_steps: usize;
    let mut counter: Vec<usize> = vec![];
    for s in start.iter() {
        loc = s.to_string();
        lr_iter = lr_line.chars();
        num_steps = 0;
        while !loc.ends_with("Z") {
            match lr_iter.next() {
                Some('L') => {
                    num_steps = num_steps + 1;
                    loc = map.get(&loc).unwrap().0.clone();
                },
                Some('R') => {
                    num_steps = num_steps + 1;
                    loc = map.get(&loc).unwrap().1.clone();
                },
            _   => {
                    lr_iter = lr_line.chars();
                },
            }
        }
        counter.push(num_steps);
    }
    let mut ans: usize = 1;
    for c in counter {
        ans = lcm(ans, c);
    }
    println!("{}", ans);
}

fn get_start(map: &HashMap<String,(String, String)>) -> Vec<String> {
    let mut start = vec![];
    for (k,_) in map.iter() {
        if k.ends_with("A") {
            start.push(k.to_string());
        }
    }
    start
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


use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/09.txt";
    let lines = read_lines(filename).unwrap();
    let mut total: i32 = 0;
    for line in lines {
        if let Ok(txt) = line {
            let seq: Vec<i32> = txt.split_whitespace().map(|t| t.parse::<i32>().unwrap()).collect();
            let val = sequence_value(&seq, vec![]);
            total = total + val;
        }
    }
    println!("Solution: {}", total);
}

fn sequence_value(seq: &Vec<i32>, mut acc: Vec<i32>) -> i32 {
    if is_terminal(seq) {
        return acc.iter().rev().fold(0, |acc, x| {x - acc})
    }
    let mut new_seq: Vec<i32> = vec![];
    for i in 0..(seq.len() - 1) {
        new_seq.push(seq[i+1] - seq[i]);
    }
    acc.push(seq[0]);
    sequence_value(&new_seq, acc)
}

fn is_terminal(seq: &Vec<i32>) -> bool {
    seq.iter().fold(true, |acc, elem| (elem == &0) && acc)
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}
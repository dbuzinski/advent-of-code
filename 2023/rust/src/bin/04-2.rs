use std::fs::File;
use std::cmp::min;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/04.txt";

    let lines = read_lines(filename).unwrap();
    let mut len: usize= 0;
    let mut n_matches: Vec<usize> = vec![];
    for line in lines {
        if let Ok(txt) = line {
            let (winning, your) = parse_card(&txt);
            n_matches.push(num_matches(winning, your));
            len = len + 1;
        }
    }
    let mut card_counts: Vec<usize> = vec![1; len];
    for (i, n) in n_matches.iter().enumerate() {
        for j in min(i+1, len)..min(i+n+1, len) {
            card_counts[j] = card_counts[j] + card_counts[i]*1;
        }
    }
    let mut sum = 0;
    for c in card_counts.iter() {
        sum = sum + c;
    }
    println!("{}", sum);
}

fn parse_card(txt: &str) -> (Vec<usize>, Vec<usize>) {
    let numbers = txt.split(":").last().unwrap();
    let split_card = numbers.split("|").collect::<Vec<&str>>();
    let winning_strs = split_card[0];
    let your_strs = split_card[1];
    let winning = winning_strs.split_whitespace().map(|s| {s.parse::<usize>().unwrap()});
    let your = your_strs.split_whitespace().map(|s| {s.parse::<usize>().unwrap()});
    (winning.collect(), your.collect())
}

fn num_matches(winning: Vec<usize>, your: Vec<usize>) -> usize {
    your.iter().fold(0, |acc, num| {
        match winning.contains(num) {
            true => acc + 1,
            false => acc,
        }
    })
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

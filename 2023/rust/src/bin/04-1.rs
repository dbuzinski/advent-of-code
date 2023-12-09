use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/04.txt";

    let lines = read_lines(filename).unwrap();
    let points = lines.fold(0, |acc, txt| {
        let card = txt.unwrap();
        let (winning, your) = parse_card(&card);
        acc + card_points(winning, your)
    });
    println!("Day 4 part 1: {}", points);
}

fn parse_card(txt: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers = txt.split(":").last().unwrap();
    let split_card = numbers.split("|").collect::<Vec<&str>>();
    let winning_strs = split_card[0];
    let your_strs = split_card[1];
    let winning = winning_strs.split_whitespace().map(|s| {s.parse::<u32>().unwrap()});
    let your = your_strs.split_whitespace().map(|s| {s.parse::<u32>().unwrap()});
    (winning.collect(), your.collect())
}

fn card_points(winning: Vec<u32>, your: Vec<u32>) -> u32 {
    let base: u32 = 2;
    let matches = your.iter().fold(0, |acc, num| {
        match winning.contains(num) {
            true => acc + 1,
            false => acc,
        }
    });
    match matches {
        0 => 0,
        n => base.pow(n - 1)
    }
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

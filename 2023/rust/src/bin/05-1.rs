use std::cmp::min;
use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/05.txt";

    let mut lines = read_lines(filename).unwrap();

    let seed_line = lines.next().unwrap().unwrap();
    let space_separated_seeds = seed_line.get(7..).unwrap();
    let mut seeds = space_separated_seeds.split(" ").collect::<Vec<&str>>().iter().map(|s| {s.parse::<usize>().unwrap()}).collect::<Vec<usize>>();

    let current_map: &mut Vec<Vec<usize>> = &mut Vec::new();
    
    for line in lines {
        if let Ok(txt) = line {
            if txt.contains("map") {
                seeds = seeds.iter().map(|s| update_seed(s, current_map)).collect::<Vec<usize>>();
                *current_map = vec![];
            } else if txt.contains(char::is_numeric) {
                update_map(current_map, &txt);
            }
        }
    }
    seeds = seeds.iter().map(|s| update_seed(s, current_map)).collect::<Vec<usize>>();
    let lowest_seed = seeds.iter().reduce(|acc, s| min(acc, s)).unwrap();
    println!("Day 5 part 1: {}", lowest_seed);
}

fn update_map(map: &mut Vec<Vec<usize>>, line: &str) {
    let nums = line.split(" ").collect::<Vec<&str>>().iter().map(|s| {s.parse::<usize>().unwrap()}).collect::<Vec<usize>>();
    map.push(nums)
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn update_seed(seed: &usize, map: &mut Vec<Vec<usize>>) -> usize {
    for interval in map.iter() {
        let (dest_start, src_start, range_len) = (interval[0], interval[1], interval[2]);
        if src_start <= *seed && *seed < src_start + range_len {
            return seed - src_start + dest_start;
        }
    }
    *seed
}

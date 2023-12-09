use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };
fn main() {
    let filename = "data/06.txt";
    let mut lines = read_lines(filename).unwrap();
    let time_line = lines.next().unwrap().unwrap();
    let times: Vec<usize> = get_elements(&time_line, 5);
    let dist_line = lines.next().unwrap().unwrap();
    let dists: Vec<usize> = get_elements(&dist_line, 9);
    assert!(times.len() == dists.len());
    let mut ways_to_win: Vec<usize> = vec![0; times.len()];
    for i in 0..times.len() {
        for charge_time in 0..times[i] {
            let finish_dist = (times[i] - charge_time)*charge_time;
            if finish_dist > dists[i] {
                ways_to_win[i] = ways_to_win[i] + 1;
            }
        }
    }
    let prod = ways_to_win.iter().fold(1, |acc, x| x*acc);
    println!("Day 6 part 1: {}", prod);
}

fn get_elements(line: &str, intro_len: usize) -> Vec<usize> {
    let elem_str = line.get(intro_len..).unwrap();
    elem_str.trim().split_whitespace().collect::<Vec<&str>>().iter().map(|s| s.parse::<usize>().unwrap()).collect()
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

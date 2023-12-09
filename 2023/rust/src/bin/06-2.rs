use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };
fn main() {
    let filename = "data/06.txt";
    let mut lines = read_lines(filename).unwrap();
    let time_line = lines.next().unwrap().unwrap();
    let time: usize = get_element(&time_line, 5);
    let dist_line = lines.next().unwrap().unwrap();
    let dist: usize = get_element(&dist_line, 9);
    let mut ways_to_win: usize = 0;
    for charge_time in 0..time {
            let finish_dist = (time - charge_time)*charge_time;
            if finish_dist > dist {
                ways_to_win = ways_to_win + 1;
            }
    }
    println!("Day 6 part 2: {}", ways_to_win);
}

fn get_element(line: &str, intro_len: usize) -> usize {
    let elem_str = line.get(intro_len..).unwrap();
    elem_str.trim().replace(char::is_whitespace, "").parse::<usize>().unwrap()
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

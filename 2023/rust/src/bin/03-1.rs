use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/03.txt";

    let lines = read_lines(filename).unwrap();

    let mut sum: usize= 0;

    let mut sc0: Option<Vec<usize>>;
    let mut sc1: Option<Vec<usize>> = None;
    let mut sc2: Option<Vec<usize>> = None;
    let mut ip1: Vec<(usize, usize)>;
    let mut ip2: Vec<(usize, usize)> = vec![];
    let mut prev_line: String = String::from("");
    for line in lines {
        if let Ok(txt) = line {
            let (int_pos, sc_pos) = parse_line(&txt);
            sc0 = sc1;
            sc1 = sc2;
            sc2 = Some(sc_pos);
            ip1 = ip2;
            ip2 = int_pos;
            for part_pos in get_parts(&ip1, &sc0, &sc1, &sc2).iter() {
                let part = prev_line.get(part_pos.0..part_pos.1).unwrap().parse::<usize>().unwrap();
                sum = sum + part;
            }
            prev_line = txt.to_string();
        }
    }
    sc0 = sc1;
    sc1 = sc2;
    sc2 = None;
    for part_pos in get_parts(&ip2, &sc0, &sc1, &sc2).iter() {
        let part = prev_line.get(part_pos.0..part_pos.1).unwrap().parse::<usize>().unwrap();
        sum = sum + part;
    }
    println!("Day 3 part 1: {}", sum);
}

fn parse_line(line: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut in_int = false;
    let mut int_start: usize = 0;
    let mut int_end: usize;
    let mut sc_pos: Vec<usize> = vec![];
    let mut int_pos: Vec<(usize, usize)> = vec![];
    for c in line.char_indices() {
        match c.1 {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                if !in_int {
                    in_int = true;
                    int_start = c.0;
                }
            }
            '.' => {
                if in_int {
                    int_end = c.0;
                    in_int = false;
                    int_pos.push((int_start, int_end));
                }
            },
            _ => {
                if in_int {
                    int_end = c.0;
                    in_int = false;
                    int_pos.push((int_start, int_end));
                }
                sc_pos.push(c.0);
            }
        }
    };
    if in_int {
        int_end = line.len();
        int_pos.push((int_start, int_end));
    }
    (int_pos, sc_pos)
}


fn get_parts<'a>(int_pos: &Vec<(usize, usize)>, sc0: &'a Option<Vec<usize>>, sc1: &'a Option<Vec<usize>>, sc2: &'a Option<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut parts: Vec<(usize, usize)> = vec![];
    for ip in int_pos.iter() {
        if is_part(*ip, sc0, sc1, sc2) {
            parts.push(*ip);
        }
    }
    parts
}

fn is_part(int_pos: (usize, usize), sc0: &Option<Vec<usize>>, sc1: &Option<Vec<usize>>, sc2: &Option<Vec<usize>>) -> bool {
    for scp0 in sc0.clone().unwrap_or_default().into_iter() {
        if int_pos.0.checked_sub(1).unwrap_or_default() <= scp0 && scp0 <= int_pos.1 {
            return true
        }
    }
    for scp1 in sc1.clone().unwrap_or_default().into_iter() {
        if int_pos.0.checked_sub(1).unwrap_or_default() <= scp1 && scp1 <= int_pos.1 {
            return true
        }
    }
    for scp2 in sc2.clone().unwrap_or_default().into_iter() {
        if int_pos.0.checked_sub(1).unwrap_or_default() <= scp2 && scp2 <= int_pos.1 {
            return true
        }
    }
    false
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

use std::fs::File;
use std::io::{ self, BufRead, BufReader, Lines };

fn main() {
    let filename = "data/03.txt";

    let lines = read_lines(filename).unwrap();

    let mut sum: usize= 0;

    let mut ints0: Option<Vec<(usize, usize)>>;
    let mut ints1: Option<Vec<(usize, usize)>> = None;
    let mut ints2: Option<Vec<(usize, usize)>> = None;
    let mut line0: String;
    let mut line1: String = String::from("");
    let mut line2: String = String::from("");
    let mut gears1: Vec<usize>;
    let mut gears2: Vec<usize> = vec![];
    for line in lines {
        if let Ok(txt) = line {
            let (int_pos, gear_pos) = parse_line(&txt);
            line0 = line1;
            line1 = line2;
            line2 = txt.to_string();
            ints0 = ints1;
            ints1 = ints2;
            ints2 = Some(int_pos);
            gears1 = gears2;
            gears2 = gear_pos;
            sum = sum + get_supergear_ratio(&gears1, &ints0, &ints1, &ints2, &line0, &line1, &line2);
        }
    }
    line0 = line1;
    line1 = line2;
    line2 = String::from("");
    ints0 = ints1;
    ints1 = ints2;
    ints2 = None;
    gears1 = gears2;
    sum = sum + get_supergear_ratio(&gears1, &ints0, &ints1, &ints2, &line0, &line1, &line2);
    println!("Day 3 part 2: {}", sum);
}

fn parse_line(line: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut in_int = false;
    let mut int_start: usize = 0;
    let mut int_end: usize;
    let mut gear_pos: Vec<usize> = vec![];
    let mut int_pos: Vec<(usize, usize)> = vec![];
    for c in line.char_indices() {
        match c.1 {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                if !in_int {
                    in_int = true;
                    int_start = c.0;
                }
            }
            '*' => {
                if in_int {
                    int_end = c.0;
                    in_int = false;
                    int_pos.push((int_start, int_end));
                }
                gear_pos.push(c.0);
            },
            _ => {
                if in_int {
                    int_end = c.0;
                    in_int = false;
                    int_pos.push((int_start, int_end));
                }
            },
        }
    };
    if in_int {
        int_end = line.len();
        int_pos.push((int_start, int_end));
    }
    (int_pos, gear_pos)
}


fn get_supergear_ratio<'a>(gear_pos: &Vec<usize>, int_pos0: &'a Option<Vec<(usize, usize)>>, int_pos1: &'a Option<Vec<(usize, usize)>>, int_pos2: &'a Option<Vec<(usize, usize)>>, line0: &str, line1: &str, line2: &str) -> usize {
    let mut ratio: usize = 0;
    for gp in gear_pos.iter() {
        ratio = ratio + get_ratio(*gp, int_pos0, int_pos1, int_pos2, line0, line1, line2);
    }
    ratio
}

fn get_ratio<'a>(gear_pos: usize, int_pos0: &Option<Vec<(usize, usize)>>, int_pos1: &'a Option<Vec<(usize, usize)>>, int_pos2: &'a Option<Vec<(usize, usize)>>, line0: &str, line1: &str, line2: &str) -> usize {
    let mut adj_gears: Vec<usize> = vec![];
    for ip0 in int_pos0.clone().unwrap_or_default() {
        if ip0.0.checked_sub(1).unwrap_or_default() <= gear_pos && gear_pos <= ip0.1 {
            let num = line0.get(ip0.0..ip0.1).unwrap().parse::<usize>().unwrap();
            adj_gears.push(num);
        }
    }
    for ip1 in int_pos1.clone().unwrap_or_default() {
        if ip1.0.checked_sub(1).unwrap_or_default() <= gear_pos && gear_pos <= ip1.1 {
            let num = line1.get(ip1.0..ip1.1).unwrap().parse::<usize>().unwrap();
            adj_gears.push(num);
        }
    }
    for ip2 in int_pos2.clone().unwrap_or_default() {
        if ip2.0.checked_sub(1).unwrap_or_default() <= gear_pos && gear_pos <= ip2.1 {
            let num = line2.get(ip2.0..ip2.1).unwrap().parse::<usize>().unwrap();
            adj_gears.push(num);
        }
    }
    if adj_gears.len() == 2 {
        return adj_gears[0] * adj_gears[1];
    }
    0
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

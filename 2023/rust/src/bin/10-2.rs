use std::fs::{ read_to_string, File };
use std::io::{self, BufRead, BufReader, Lines};

use num::Integer;


fn main() {
    let filepath = "data/10.txt";
    let map = read_to_string(filepath)
        .expect("Failed to read input file");

    let line_length = map.find("\n").unwrap() + 1;
    let starting_point: i32 = map.find("S").unwrap().try_into().unwrap();
    let boundary = walk(&map, starting_point, Direction::Start, line_length, vec![]).expect("Failed to calculate boundary");

    let lines = read_lines(filepath).unwrap();
    let mut periods_inside_loop = 0;
    for (line_num, line) in lines.enumerate() {
        if let Ok(txt) = line {
            let number_inner_in_line = count_area(&txt, line_num, &boundary);
            periods_inside_loop = periods_inside_loop + number_inner_in_line;
        }
    }
    println!("Day 10 part 2: {}", periods_inside_loop);
}

// Count the number of periods inside the loop where | and - are linear pipes and
// 7, F, L, and J are 90 degree turns that form the loop boundary, S is the loop starting point,
// and '.' is a period.
fn count_area(line: &str, line_num: usize, boundary: &Vec<(usize, usize)>) -> u32 {
    let mut periods = 0;
    let mut inside_loop = false;
    for (i, c) in line.chars().enumerate() {
        match c {
            'L' | 'J' | '|' | 'S' => {
                if boundary.contains(&(i, line_num)) {
                    inside_loop = !inside_loop;
                } else if inside_loop {
                    periods = periods + 1;
                }
            },
            _ => {
                if inside_loop && !boundary.contains(&(i, line_num)) {
                    periods = periods + 1;
                }
            }
        }
    };
    periods
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

fn walk(map: &str, current_point: i32, direction: Direction, line_length: usize, mut boundary: Vec<(usize, usize)>) -> Result<Vec<(usize, usize)>, &str> {
    let len: i32 = line_length.try_into().unwrap();
    let current_char = map.chars().nth(current_point.try_into().unwrap()).unwrap();
    let nx: usize = current_point.try_into().unwrap();
    let ny: usize = current_point.try_into().unwrap();
    let xy_coords: (usize, usize) = (nx.mod_floor(&line_length), ny.div_floor(&line_length));
    boundary.push(xy_coords);
    match current_char {
        '|' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point + len, direction, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point - len, direction, line_length, boundary);
                }
            }
        },
        '-' => {
            match direction {
                Direction::Right => {
                    return walk(map, current_point + 1, direction, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point - 1, direction, line_length, boundary);
                }
            }
        },
        'L' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point + 1, Direction::Right, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point - len, Direction::Up, line_length, boundary);
                }
            }
        },
        'J' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point - 1, Direction::Left, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point - len, Direction::Up, line_length, boundary);
                }
            }
        },
        'F' => {
            match direction {
                Direction::Up => {
                    return walk(map, current_point + 1, Direction::Right, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point + len, Direction::Down, line_length, boundary);
                }
            }
        },
        '7' => {
            match direction {
                Direction::Up => {
                    return walk(map, current_point - 1, Direction::Left, line_length, boundary);
                },
                _ => {
                    return walk(map, current_point + len, Direction::Down, line_length, boundary);
                }
            }
        },
        'S' => {
            if boundary.len() > 1 {
                return Ok(boundary);
            } else {
                let current_ind: i32 = current_point.try_into().unwrap();
                let left = map.chars().nth((current_ind - 1).try_into().unwrap()).unwrap();
                let down = map.chars().nth((current_ind + len).try_into().unwrap()).unwrap();
                let right = map.chars().nth((current_ind + 1).try_into().unwrap()).unwrap();
                if left == '-' {
                    return walk(map, current_point - 1, Direction::Left, line_length, boundary);
                } else if left == 'F' {
                    return walk(map, current_point - 1, Direction::Left, line_length, boundary);
                } else if down == '|' {
                    return walk(map, current_point + len, Direction::Down, line_length, boundary);
                } else if down == 'L' {
                    return walk(map, current_point + len, Direction::Down, line_length, boundary);
                } else if right == '-' {
                    return walk(map, current_point + 1, Direction::Right, line_length, boundary);
                } else if right == '7' {
                    return walk(map, current_point + 1, Direction::Right, line_length, boundary);
                } else {
                    return Err("Encountered unexpected character");
                }
            }
        },
        _ => Err("Encountered unexpected character"),
    }
}


fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

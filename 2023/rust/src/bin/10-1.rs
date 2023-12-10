use std::fs::read_to_string;

fn main() {
    let filepath = "data/10.txt";
    let map = read_to_string(filepath)
        .expect("Failed to read input file");

    let line_length = map.find("\n").unwrap() + 1;
    let starting_point: i32 = map.find("S").unwrap().try_into().unwrap();
    let pipe_len = walk(&map, starting_point, Direction::Start, line_length, 0);
    println!("Day 10 part 1: {}", pipe_len.expect("Unable to find pipe length").div_ceil(2));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

fn walk(map: &str, current_point: i32, direction: Direction, line_length: usize, walk_len: usize) -> Result<usize, &str> {
    let len: i32 = line_length.try_into().unwrap();
    let current_char = map.chars().nth(current_point.try_into().unwrap()).unwrap();
    match current_char {
        '|' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point + len, direction, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point - len, direction, line_length, walk_len + 1);
                }
            }
        },
        '-' => {
            match direction {
                Direction::Right => {
                    return walk(map, current_point + 1, direction, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point - 1, direction, line_length, walk_len + 1);
                }
            }
        },
        'L' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point + 1, Direction::Right, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point - len, Direction::Up, line_length, walk_len + 1);
                }
            }
        },
        'J' => {
            match direction {
                Direction::Down => {
                    return walk(map, current_point - 1, Direction::Left, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point - len, Direction::Up, line_length, walk_len + 1);
                }
            }
        },
        'F' => {
            match direction {
                Direction::Up => {
                    return walk(map, current_point + 1, Direction::Right, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point + len, Direction::Down, line_length, walk_len + 1);
                }
            }
        },
        '7' => {
            match direction {
                Direction::Up => {
                    return walk(map, current_point - 1, Direction::Left, line_length, walk_len + 1);
                },
                _ => {
                    return walk(map, current_point + len, Direction::Down, line_length, walk_len + 1);
                }
            }
        },
        'S' => {
            if walk_len > 0 {
                return Ok(walk_len);
            } else {
                let current_ind: i32 = current_point.try_into().unwrap();
                let left = map.chars().nth((current_ind - 1).try_into().unwrap()).unwrap();
                let down = map.chars().nth((current_ind + len).try_into().unwrap()).unwrap();
                let right = map.chars().nth((current_ind + 1).try_into().unwrap()).unwrap();
                if left == '-' {
                    return walk(map, current_point - 1, Direction::Left, line_length, walk_len + 1);
                } else if left == 'F' {
                    return walk(map, current_point - 1, Direction::Left, line_length, walk_len + 1);
                } else if down == '|' {
                    return walk(map, current_point + len, Direction::Down, line_length, walk_len + 1);
                } else if down == 'L' {
                    return walk(map, current_point + len, Direction::Down, line_length, walk_len + 1);
                } else if right == '-' {
                    return walk(map, current_point + 1, Direction::Right, line_length, walk_len + 1);
                } else if right == '7' {
                    return walk(map, current_point + 1, Direction::Right, line_length, walk_len + 1);
                } else {
                    return Err("Encountered unexpected character");
                }
            }
        },
        _ => Err("Encountered unexpected character"),
    }
}

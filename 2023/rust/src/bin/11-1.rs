use std::fs::read_to_string;

fn main() {
    let filepath = "data/11.txt";
    let map = read_to_string(filepath)
        .expect("Failed to read input file");
   
    let expanded_map = expand_columns(&expand_rows(&map));
    let galaxy_coordinates = find_galaxy_coordinates(&expanded_map);

    let mut total_distance = 0;
    for (i, galaxy1) in galaxy_coordinates.iter().enumerate() {
        for galaxy2 in galaxy_coordinates.iter().skip(i + 1) {
            total_distance += find_distance(*galaxy1, *galaxy2);
        }
    }

    println!("Day 11 part 1: {}", total_distance);
}

fn find_distance(galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
    let row_distance = if galaxy1.0 > galaxy2.0 {
        galaxy1.0 - galaxy2.0
    } else {
        galaxy2.0 - galaxy1.0
    };
    let column_distance = if galaxy1.1 > galaxy2.1 {
        galaxy1.1 - galaxy2.1
    } else {
        galaxy2.1 - galaxy1.1
    };
    row_distance + column_distance
}

fn find_empty_rows(map: &str) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut row_num = 0;
    for row in map.lines() {
        if row.find('#').is_none() {
            empty_rows.push(row_num);
        }
        row_num += 1;
    }
    empty_rows
}

fn find_empty_columns(map: &str) -> Vec<usize> {
    let mut empty_columns: Vec<usize> = Vec::new();
    let mut column_num = 0;
    let mut empty_column = true;
    for _ in map.lines().next().unwrap().chars() {
        for row in map.lines() {
            if row.chars().nth(column_num).unwrap() == '#' {
                empty_column = false;
                break;
            }
        }
        if empty_column {
            empty_columns.push(column_num);
        }
        column_num += 1;
        empty_column = true;
    }
    empty_columns
}

// add a 2nd empty column where all empty columns are
fn expand_columns(map: &str) -> String {
    let mut expanded_map = String::new();
    let empty_columns = find_empty_columns(&map);
    for row in map.lines() {
        let mut column_num = 0;
        for column in row.chars() {
            if empty_columns.contains(&column_num) {
                expanded_map.push(column);
                expanded_map.push('.');
            } else {
                expanded_map.push(column);
            }
            column_num += 1;
        }
        expanded_map.push_str("\n");
    }
    expanded_map
}

fn expand_rows(map: &str) -> String {
    let mut expanded_map = String::new();
    let empty_rows = find_empty_rows(&map);
    let mut row_num = 0;
    for row in map.lines() {
        if empty_rows.contains(&row_num) {
            expanded_map.push_str(row);
            expanded_map.push_str("\n");
            expanded_map.push_str(row);
            expanded_map.push_str("\n");
        } else {
            expanded_map.push_str(row);
            expanded_map.push_str("\n");
        }
        row_num += 1;
    }
    expanded_map
}

fn find_galaxy_coordinates(map: &str) -> Vec<(usize, usize)> {
    let mut galaxy_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut row_num = 0;
    for row in map.lines() {
        let mut column_num = 0;
        for column in row.chars() {
            if column == '#' {
                galaxy_coordinates.push((row_num, column_num));
            }
            column_num += 1;
        }
        row_num += 1;
    }
    galaxy_coordinates
}


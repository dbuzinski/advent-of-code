use std::fs::read_to_string;

fn main() {
    let filepath = "data/11.txt";
    let map = read_to_string(filepath)
        .expect("Failed to read input file");
   
    let galaxy_coordinates = find_galaxy_coordinates(&map);
    let empty_rows = find_empty_rows(&map);
    let empty_columns = find_empty_columns(&map);

    let mut total_distance = 0;
    let scaling_factor = 1000000;
    let mut ind = 0;
    for (i, galaxy1) in galaxy_coordinates.iter().enumerate() {
        for galaxy2 in galaxy_coordinates.iter().skip(i + 1) {
            ind = ind + 1;
            total_distance += find_distance(*galaxy1, *galaxy2, &empty_rows, &empty_columns, scaling_factor);
        }
    }
    println!("Day 11 part 2: {}", total_distance);
}

fn find_distance(galaxy1: (usize, usize), galaxy2: (usize, usize), empty_rows: &Vec<usize>, empty_columns: &Vec<usize>, scaling_factor: usize) -> usize {
    let (row1, column1) = galaxy1;
    let (row2, column2) = galaxy2;
    let row_distance: usize;
    if row1 < row2 {
        row_distance = row2 - row1;
    } else {
        row_distance = row1 - row2;
    }
    let column_distance: usize;
    if column1 < column2 {
        column_distance = column2 - column1;
    } else {
        column_distance = column1 - column2;
    }
    let mut empty_rows_between = 0;
    for row in empty_rows.iter() {
        if *row > row1 && *row < row2 {
            empty_rows_between += 1;
        } else if *row > row2 && *row < row1 {
            empty_rows_between += 1;
        }
    }
    let mut empty_columns_between = 0;
    for column in empty_columns.iter() {
        if *column > column1 && *column < column2 {
            empty_columns_between += 1;
        } else if *column > column2 && *column < column1 {
            empty_columns_between += 1;
        }
    }
    let distance = row_distance + column_distance  + empty_rows_between * scaling_factor + empty_columns_between * scaling_factor - empty_rows_between - empty_columns_between;
    distance
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


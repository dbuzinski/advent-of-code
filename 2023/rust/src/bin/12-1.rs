use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    let filename = "data/12.txt";
    let lines = read_lines(filename).unwrap();
    let mut n_arrangements = 0;
    for line in lines {
        if let Ok(content) = line {
            let (template, counts) = parse_line(&content);
            let new_arrangements = find_arrangements(template, &counts);
            n_arrangements = n_arrangements + new_arrangements;
        }
    }
    println!("Day 12, Part 1: {}", n_arrangements);
}

fn fill_template(template: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut n_question_mark = 0;
    for c in template.chars() {
        if c == '?' {
            n_question_mark += 1;
        }
    }
    let n = 2u32.pow(n_question_mark);
    for i in 0..n {
        let mut s = String::new();
        let mut j = 0;
        for c in template.chars() {
            if c == '?' {
                if i & (1 << j) != 0 {
                    s.push('#');
                } else {
                    s.push('.');
                }
                j += 1;
            } else {
                s.push(c);
            }
        }
        result.push(s);
    }
    result
}

fn find_arrangements(template: &str, counts: &[usize]) -> usize {
    fill_template(template).iter().filter(|x| satisfies_counts(x, counts)).count()
}

fn satisfies_counts(line: &str, counts: &[usize]) -> bool {
    let line_counts: &[&str] = &line.split('.').filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    if line_counts.len() != counts.len() {
        return false;
    }
    for (line_count, count) in line_counts.iter().zip(counts.iter()) {
        if line_count.len() != *count {
            return false;
        }
    }
    true
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let mut iter = line.split_whitespace();
    let template = iter.next().unwrap();
    let counts = iter.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    (template, counts)
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

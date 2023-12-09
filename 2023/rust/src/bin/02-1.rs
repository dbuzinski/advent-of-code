use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;

fn main() {
    let filepath = "data/02.txt";
    let lines = read_lines(filepath).unwrap();

    let mut s = 0;

    for line in lines {
        if let Ok(txt) = line {
            let (id, game) = game_info(&txt).unwrap();
            if is_valid(game) {
                s = s + id;
            }
        }
    }
    println!("Day 2 part 1: {}", s);
}

type Game<'a> = Vec<HashMap<&'a str, u32>>;

fn game_info(txt: &str) -> Result<(u32, Game), &str> {
    let split = txt.find(":").unwrap();
    let id_str = txt.get(5..split).unwrap();
    let id = id_str.parse::<u32>().unwrap();

    let mut maps = vec![];

    let sets_str = txt.get(split+1..).unwrap().split(";");
    for set in sets_str {
        maps.push(parse_set(set));
    }
    Ok((id, maps))
}

fn is_valid(game: Game) -> bool {
    let tf: bool = game.iter().fold(true, |acc, set| is_valid_set(set) && acc);
    tf
}

fn is_valid_set(set: &HashMap<&str, u32>) -> bool {
    set.get("green").unwrap_or(&0) <= &13 && set.get("blue").unwrap_or(&0) <= &14 && set.get("red").unwrap_or(&0) <= &12
}

fn parse_set(txt: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    let color = txt.split(",");
    for balls in color {
        match balls.trim() {
            s if s.ends_with("green") => {
                let num_str = s.get(..s.len()-6).unwrap();
                let num = num_str.parse::<u32>().unwrap();
                map.insert("green", num)
            },
            s if s.ends_with("blue") => {
                let num_str = s.get(..s.len()-5).unwrap();
                let num = num_str.parse::<u32>().unwrap();
                map.insert("blue", num)

            },
            s if s.ends_with("red") => {
                let num_str = s.get(..s.len()-4).unwrap();
                let num = num_str.parse::<u32>().unwrap();
                map.insert("red", num)

            },
            _ => None,
        };
    }
    map
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines)
}

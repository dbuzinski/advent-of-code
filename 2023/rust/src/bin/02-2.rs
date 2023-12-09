use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;

fn main() {
    let filepath = "data/02.txt";
    let lines = read_lines(filepath).unwrap();

    let mut s = 0;

    for line in lines {
        if let Ok(txt) = line {
            let (_, game) = game_info(&txt).unwrap();
            s = s + power(game);
        }
    }
    println!("Day 2 part 2: {}", s);
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

fn power(game: Game) -> u32 {
    let init: HashMap<&str, u32> = HashMap::from([("green", 0), ("blue", 0), ("red", 0)]);
    let min = game.iter().fold(init, |acc, set| min_set(&acc, &set));
    min.get("green").unwrap_or(&0) * min.get("blue").unwrap_or(&0) * min.get("red").unwrap_or(&0)
}


fn min_set<'a>(acc: &HashMap<&'a str, u32>, set: &HashMap<&'a str, u32>) -> HashMap<&'a str, u32> {
    let acc_green = acc.get("green").unwrap_or(&0);
    let acc_blue = acc.get("blue").unwrap_or(&0);
    let acc_red = acc.get("red").unwrap_or(&0);
    let set_green = set.get("green").unwrap_or(&0);
    let set_blue = set.get("blue").unwrap_or(&0);
    let set_red = set.get("red").unwrap_or(&0);
    HashMap::from([("green", *acc_green.max(set_green)), ("blue", *acc_blue.max(set_blue)), ("red", *acc_red.max(set_red))])
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

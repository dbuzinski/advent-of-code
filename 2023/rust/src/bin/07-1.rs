use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::cmp::Reverse;

fn main() {
    let filepath = "data/07.txt";

    let lines = read_lines(filepath).unwrap();
    let mut hands: Vec<(String, usize)> = vec![];
    for line in lines {
        if let Ok(txt) = line {
            hands.push(get_hand(&txt));
        }
        hands.sort_by(|h1, h2| compare_hands(&h1.0, &h2.0));
    }
    let mut total = 0;
    for (placing, hand) in hands.iter().enumerate() {
        total = total + (placing+1)*hand.1;
    }
    println!("{}", total);
}

fn get_hand(line: &str) -> (String, usize) {
    let mut spl = line.split(" ");
    let hand = spl.next().unwrap();
    let bid = spl.next().unwrap().parse::<usize>().unwrap();
    (hand.to_string(), bid)
}

fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    let hs1 = get_hand_score(hand1);
    let hs2 = get_hand_score(hand2);
    if hs1 < hs2 {
        return Ordering::Less;
    } else if hs1 > hs2 {
        return  Ordering::Greater;
    } else {
        for i in 0..5 {
            let cs1 = card_score(hand1.chars().nth(i).unwrap());
            let cs2 = card_score(hand2.chars().nth(i).unwrap());
            if cs1 < cs2 {
                return Ordering::Less;
            } else if cs1 > cs2 {
                return Ordering::Greater;
            }
        }
    }
    Ordering::Equal
}

fn card_score(c: char) -> Result<usize, &'static str> {
    match c {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(11),
        'T' => Ok(10),
        '9' => Ok(9),
        '8' => Ok(8),
        '7' => Ok(7),
        '6' => Ok(6),
        '5' => Ok(5),
        '4' => Ok(4),
        '3' => Ok(3), 
        '2' => Ok(2),
        _ => Err("Unknown character")
    }
}

fn get_hand_score(hand: &str) -> usize {
    let mut counts: HashMap<char, usize>= HashMap::new();
    for card in hand.chars() {
        if let Some(x) = counts.get_mut(&card) {
            *x = *x + 1;
        } else {
            counts.insert(card, 1);
        }
    }
    let mut strengths: Vec<usize> = counts.into_values().collect();
    strengths.sort_by_key(|k| Reverse(*k));
    let mut score = 1;
    if strengths[0] == 5 {
        score = 7;
    } else if strengths[0] == 4 {
        score = 6;
    } else if strengths[0] == 3 && strengths[1] == 2 {
        score = 5;
    } else if strengths[0] == 3 && strengths[1] == 1 {
        score = 4;
    } else if strengths[0] == 2 && strengths[1] == 2 {
        score = 3;
    } else if strengths[0] == 2 && strengths[1] == 1 {
        score = 2;
    }
    score
}

fn read_lines(filename: &str) -> io::Result<Lines<BufReader<File>>>{
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines)
}


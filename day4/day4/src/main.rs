use std::{collections::{HashSet, HashMap}, fs};

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut multiples_map: HashMap<u32, u32> = HashMap::new();

    let mut total = 0;
    let mut game = 1;
    for line in contents.lines() {
        let (winners, haves) = parse_line(line);
        let matches = find_matches(winners, haves);

        let multiples_of_cur = *multiples_map.entry(game).or_insert(1);

        for i in game+1..=game+matches {
            let other = multiples_map.entry(i).or_insert(1);
            *other += multiples_of_cur;
        }

        total += multiples_of_cur;
        game += 1;
    }

    println!("{}", total);
}

fn find_matches(winners: HashSet<u32>, haves: Vec<u32>) -> u32 {
    let mut total = 0;
    for have in haves {
        total += match winners.contains(&have) {
            true => 1,
            false => 0,
        }
    }

    total
}

fn parse_line(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let mut parts = line.split(": ");
    parts.next().unwrap();
    let pertinent = parts.next().unwrap();
    let mut pertinents = pertinent.split(" | ");

    let winners_str = pertinents.next().unwrap();
    let haves_str = pertinents.next().unwrap();

    let mut winners: HashSet<u32> = HashSet::new();
    for card in winners_str.split(" ") {
        match card.parse::<u32>() {
            Ok(x) => {
                winners.insert(x);
            }
            Err(_) => (),
        }
    }

    let mut haves = Vec::new();
    for card in haves_str.split(" ") {
        match card.parse::<u32>() {
            Ok(x) => haves.push(x),
            Err(_) => (),
        }
    }

    (winners, haves)
}

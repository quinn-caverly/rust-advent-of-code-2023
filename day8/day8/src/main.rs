use std::{collections::HashMap, fs};

fn main() {
    let (input_turns, map) = parse_input();

    let mut i = 0;
    let mut curs = create_curs(&map);

    println!("{}", curs.len());

    let mut steps = 0;
    loop {
        steps += 1;

        if i == input_turns.len() {
            i = 0;
        }

        let mut new_curs = Vec::new();
        let mut all_cur = true;
        let cur_input = input_turns.get(i).unwrap();

        for cur in curs {
            let (left, right) = map.get(&cur).unwrap();

            let new_cur = match cur_input {
                Turn::Left => left.clone(),
                Turn::Right => right.clone(),
            };

            if new_cur != "ZZZ".to_string() {
                all_cur = false;
            }

            new_curs.push(new_cur);
        }

        if all_cur {
            break;
        }

        curs = new_curs;
        i += 1;

        if steps % 1000000 == 0 {
            println!("{}", steps);
        }
    }

    println!("{}", steps);
}

fn create_curs(map: &HashMap<String, (String, String)>) -> Vec<String> {
    let mut curs = Vec::new();

    for start in map.keys() {
        if start.chars().nth(2).unwrap() == 'A' {
            curs.push(start.clone());
        }
    }

    curs
}

enum Turn {
    Left,
    Right,
}

fn parse_input() -> (Vec<Turn>, HashMap<String, (String, String)>) {
    let contents = fs::read_to_string("../input.txt").unwrap();
    let mut lines = contents.lines();

    let input = lines.next().unwrap();

    let mut input_turns = Vec::new();
    for ch in input.chars() {
        if ch == 'L' {
            input_turns.push(Turn::Left);
        } else if ch == 'R' {
            input_turns.push(Turn::Right);
        }
    }

    lines.next().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let mut portions = line.split(" = ");

        let start = portions.next().unwrap().to_string();

        let mut end = portions.next().unwrap();
        end = end.strip_prefix("(").unwrap();
        end = end.strip_suffix(")").unwrap();

        let mut left_and_right = end.split(", ");

        let (left, right) = (
            left_and_right.next().unwrap().to_string(),
            left_and_right.next().unwrap().to_string(),
        );

        map.insert(start, (left, right));
    }

    (input_turns, map)
}

use std::{fs, collections::HashMap};

fn main() {
    part_one();
}

fn parse_input() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("../input.txt").unwrap();

    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

// we can tell that S should be a 'J'
fn part_one() {
    let input = parse_input();
    let (row_idx, col_idx) = find_s(&input).unwrap();

    let mut distances_map: HashMap<(usize, usize), u32> = HashMap::new();

    // we need to go both north and left
    let mut opt = step(row_idx, col_idx, Direction::North, &input);
    let mut step = 1;
    while opt.is_some() {

    }
}

fn run() {

}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn step(row_idx: usize, col_idx: usize, dir: Direction, input: &Vec<Vec<char>>) -> Option<(usize, usize, Direction)> {
    // for this, I'll assume that the board is parsed correctly
    return match input.get(row_idx).unwrap().get(col_idx).unwrap() {
        '|' => {
            if dir == Direction::North {
                Some((row_idx-1, col_idx, Direction::North))
            } else {
                Some((row_idx+1, col_idx, Direction::South))
            }
        },
        '-' => {
            if dir == Direction::East {
                Some((row_idx, col_idx+1, Direction::East))
            } else {
                Some((row_idx, col_idx-1, Direction::West))
            }
        },
        'L' => {
            if dir == Direction::South {
                Some((row_idx, col_idx+1, Direction::East))
            } else {
                Some((row_idx-1, col_idx, Direction::North))
            }
        },
        'J' => {
            if dir == Direction::South {
                Some((row_idx, col_idx-1, Direction::West))
            } else {
                Some((row_idx-1, col_idx, Direction::North))
            }
        },
        '7' => {
            if dir == Direction::East {
                Some((row_idx+1, col_idx, Direction::South))
            } else {
                Some((row_idx, col_idx-1, Direction::West))
            }
        },
        'F' => {
            if dir == Direction::North {
                Some((row_idx, col_idx+1, Direction::East))
            } else {
                Some((row_idx+1, col_idx, Direction::South))
            }
        },
        'S' => {
            None
        },
        _ => panic!("Did not expect to be on this square: {}, {}", row_idx, col_idx),
    }
}

fn find_s(input: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &elem) in row.iter().enumerate() {
            if elem == 'S' {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

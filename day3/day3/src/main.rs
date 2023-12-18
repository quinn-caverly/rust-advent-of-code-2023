use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let vec_deques = parse_input();
    let rows = vec_deques.len();
    let columns = vec_deques.get(0).unwrap().len();
    let grid = create_grid(vec_deques);

    let part_numbers = find_part_numbers(rows as i32, columns as i32, &grid);

    let total = get_gear_sum(rows as i32, columns as i32, &grid, part_numbers);

    println!("{}", total);
}

#[derive(Clone, Copy)]
struct PartNumber {
    row: i32,
    start_column: i32,
    end_column: i32,
    value: u32,
}

fn get_gear_sum(
    rows: i32,
    columns: i32,
    grid: &HashMap<(i32, i32), char>,
    part_numbers: Vec<PartNumber>,
) -> u32 {
    let mut gear_ratio_total = 0;

    for row in 0..rows {
        for column in 0..columns {
            let cur = grid.get(&(row, column)).unwrap();

            if *cur == '*' {
                let adj_part_numbers = get_adjacent_part_numbers(row, column, &part_numbers);

                if adj_part_numbers.len() == 2 {
                    gear_ratio_total += adj_part_numbers.get(0).unwrap().value
                        * adj_part_numbers.get(1).unwrap().value;
                }
            }
        }
    }

    gear_ratio_total
}

fn get_adjacent_part_numbers(
    row: i32,
    column: i32,
    part_numbers: &Vec<PartNumber>,
) -> Vec<PartNumber> {
    let mut adj_part_numbers = Vec::new();

    'part_numbers: for part_number in part_numbers {
        // for each coordinate in the part number, is it adjacent to the gear?
        // or is abs distance in x <= 1 and abs distance in y <= 1?
        for part_column in part_number.start_column..=part_number.end_column {
            let y_dist = (column - part_column).abs();
            let x_dist = (row - part_number.row).abs();

            if x_dist <= 1 && y_dist <= 1 {
                adj_part_numbers.push(*part_number);
                continue 'part_numbers;
            }
        }
    }

    adj_part_numbers
}

fn find_part_numbers(rows: i32, columns: i32, grid: &HashMap<(i32, i32), char>) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    for row in 0..rows {
        let mut currently_building_num = false;
        let mut digits: Vec<u32> = Vec::new();
        let mut lowest_index = 0;
        let mut highest_index = 0;

        for column in 0..columns {
            let cur = grid
                .get(&(row, column))
                .expect(&format!("{}, {}", row, column));

            match cur.to_digit(10) {
                Some(x) => {
                    if currently_building_num {
                        digits.push(x);
                        highest_index = column;
                    } else {
                        currently_building_num = true;
                        digits = Vec::new();
                        digits.push(x);
                        lowest_index = column;
                        highest_index = column;
                    }
                }
                None => {
                    if currently_building_num {
                        if check_if_part_number(lowest_index, highest_index, row, &grid) {
                            part_numbers.push(PartNumber {
                                row,
                                start_column: lowest_index,
                                end_column: highest_index,
                                value: build_number(&digits),
                            });
                        }
                        currently_building_num = false;
                    }
                }
            }
        }
    }

    part_numbers
}

fn build_number(digits: &Vec<u32>) -> u32 {
    let mut rev_digits = digits.clone();
    rev_digits.reverse();

    let mut multi = 1;

    let mut total = 0;
    for dig in rev_digits {
        total += dig * multi;
        multi *= 10;
    }

    total
}

fn check_if_part_number(
    lowest_index: i32,
    highest_index: i32,
    row_index: i32,
    grid: &HashMap<(i32, i32), char>,
) -> bool {
    // we check from row-1 to row+1 and for each column_indices[0]-1 to column_indices[-1]+1
    for row in row_index - 1..=row_index + 1 {
        for column in lowest_index - 1..=highest_index + 1 {
            // numbers can't be adj to numbers
            let cur = grid.get(&(row, column)).unwrap();

            if *cur != '.' {
                // we need to check if it is not the actual part number
                match (*cur).to_digit(10) {
                    Some(_) => (),
                    None => return true,
                }
            }
        }
    }

    return false;
}

fn create_grid(vec_deques: VecDeque<VecDeque<char>>) -> HashMap<(i32, i32), char> {
    let mut grid = HashMap::new();

    let mut row = 0;
    let mut column;
    for vec_deque in vec_deques {
        column = 0;
        for ch in vec_deque {
            grid.insert((row, column), ch);
            column += 1;
        }
        row += 1;
    }

    grid
}

// adds buffers so that it is impossible to be out of bounds when checking adjacents
fn parse_input() -> VecDeque<VecDeque<char>> {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut grid: VecDeque<VecDeque<char>> = VecDeque::new();

    for line in contents.lines() {
        let mut chars: VecDeque<char> = line.chars().collect();
        chars.push_front('.');
        chars.push_back('.');

        grid.push_back(chars);
    }

    let length_of_line = grid.get(0).unwrap().len();
    let mut buffer_line: VecDeque<char> = VecDeque::new();
    for _i in 0..length_of_line {
        buffer_line.push_back('.');
    }

    grid.push_front(buffer_line.clone());
    grid.push_back(buffer_line);

    grid
}

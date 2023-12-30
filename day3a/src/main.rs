use core::num;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let part_num_map: HashMap<(usize, usize), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            (0..line.len()).filter_map(move |col_idx| {
                line.get(col_idx..col_idx + 3) // all nums are 1 to 3 digits
                    .and_then(|num_str| num_str.parse::<u32>().ok())
                    .or(line.get(col_idx..col_idx + 2).and_then(|num_str| {
                        match line.chars().nth(col_idx - 1) {
                            Some(_) => None,
                            None => num_str.parse::<u32>().ok(),
                        }
                    }))
                    .or(line.get(col_idx..col_idx + 1).and_then(|num_str| {
                        match line.chars().nth(col_idx - 1) {
                            Some(_) => None,
                            None => num_str.parse::<u32>().ok(),
                        }
                    }))
                    .map(|num| ((row_idx, col_idx), num))
            })
        })
        .collect();

    let refer = &part_num_map;

    println!(
        "{}",
        input
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| ch == &'*')
                    .filter_map(move |(col_idx, _)| {
                        let part_nums = (row_idx - 1..=row_idx + 1)
                            .flat_map(|row_offset| {
                                (col_idx - 3..=col_idx + 1).filter_map(move |col_offset| {
                                    refer.get(&(row_offset, col_offset)).copied()
                                })
                            })
                            .collect::<Vec<u32>>();

                        match part_nums.len() {
                            2 => {
                                println!(
                                    "{}, {}",
                                    part_nums.get(0).unwrap(),
                                    part_nums.get(1).unwrap()
                                );
                                Some(*part_nums.get(0).unwrap() * *part_nums.get(1).unwrap())
                            }
                            _ => None,
                        }
                    })
            })
            .sum::<u32>()
    );
}

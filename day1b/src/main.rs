use std::usize;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|line| {
                (0..line.len()).find_map(|i| take_or_not(line, i)).unwrap() * 10
                    + (0..line.len())
                        .rev()
                        .find_map(|i| take_or_not(line, i))
                        .unwrap()
            })
            .sum::<usize>()
    )
}

fn take_or_not(line: &str, i: usize) -> Option<usize> {
    line.chars().nth(i).and_then(|c| {
        if c.is_digit(10) {
            Some(c.to_digit(10).unwrap() as usize)
        } else {
            NUMS.iter()
                .enumerate()
                .find(|(_, &word)| line[i..].starts_with(word))
                .map(|(idx, _)| idx + 1)
        }
    })
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut total = 0;

    for line in contents.lines() {
        let start = {
            let (dig_index, dig_val) = find_digit_digit(line);
            let (exists, word_index, word_val) = find_string_digit(line, true);


            if exists {
                if dig_index < word_index {
                    dig_val
                } else {
                    word_val
                }
            } else {
                dig_val
            }
        };

        let end = {
            let reverse_line: String = line.chars().rev().collect();
            let (mut dig_index, dig_val) = find_digit_digit(&reverse_line);
            dig_index = line.len() - 1 - dig_index; // because we want from the end

            let (exists, word_index, word_val) = find_string_digit(line, false);

            println!("word-index: {}, word-val: {}, dig_index: {}, dig_val: {}", word_index, word_val, dig_index, dig_val);

            if exists {
                if dig_index > word_index {
                    dig_val
                } else {
                    word_val
                }
            } else {
                dig_val
            }
        };

        let cur = start * 10 + end;
        total += cur;
    }

    println!("{}", total);
}

fn find_digit_digit(line: &str) -> (usize, u32) {
    let mut i: usize = 0;
    for dig in line.chars() {
        match dig.to_digit(10) {
            Some(x) => return (i, x),
            None => (),
        }
        i += 1;
    }
    panic!("Did not expect there to a be line with no digit ex) 1");
}

fn find_string_digit(line: &str, want_first: bool) -> (bool, usize, u32) {
    let array: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first_elem = "uninitialized";
    let mut index = if want_first { line.len() } else { 0 };

    for elem in array {
        if line.contains(elem) {
            // we need to find the index where it starts
            let size = elem.len();

            for i in 0..=line.len() - size {
                let cur_slice = &line[i..i + size];

                if cur_slice == elem {
                    if (want_first && i < index) || (!want_first && i > index) {
                        first_elem = elem;
                        index = i;
                    }
                }
            }
        }
    }

    if first_elem == "uninitialized" {
        return (false, 0, 0);
    } else {
        let val = match first_elem {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("did not expect digit not 1 thru 9"),
        };
        return (true, index, val);
    }
}

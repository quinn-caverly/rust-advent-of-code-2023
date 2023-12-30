use std::fs;

fn main() {
    let sequences = parse_input();

    let mut sum = 0;

    for sequence in sequences {
        let mut build: Vec<Vec<i32>> = Vec::new();
        build.push(sequence.clone());

        // build down
        while !is_row_all_zero(build.get(build.len() - 1).unwrap()) {
            let row_below = get_row_below(build.get(build.len() - 1).unwrap());
            build.push(row_below);
        }

        println!("Built Down: ");
        for line in &build {
            println!("{:?}", line);
        }
        println!();

        // build back up
        let build_len = build.len();
        build.get_mut(build_len - 1).unwrap().push(0);

        let mut cur_extrapolation_index = build_len - 2;
        while cur_extrapolation_index > 0 {
            extrapolate_row_i(cur_extrapolation_index, &mut build);
            cur_extrapolation_index -= 1;
        }

        extrapolate_row_i(0, &mut build);

        println!("Built Up: ");
        for line in &build {
            println!("{:?}", line);
        }
        println!("{}", build
                    .get(0)
                    .unwrap()
                    .get(build.get(0).unwrap().len() - 1)
                    .unwrap()
        );

        // once we are done, we add the final value in the 0th row to the sum
        sum += build
            .get(0)
            .unwrap()
            .get(build.get(0).unwrap().len() - 1)
            .unwrap();
    }

    println!("{}", sum);
}

fn is_row_all_zero(cur: &Vec<i32>) -> bool {
    for val in cur {
        if val != &0 {
            return false;
        }
    }
    return true;
}

fn extrapolate_row_i(i: usize, seq: &mut Vec<Vec<i32>>) {
    let last_elem_index = seq.get(i).unwrap().len() - 1;

    let new_val = seq.get(i).unwrap().get(last_elem_index).unwrap()
        + seq
            .get((i + 1) as usize)
            .unwrap()
            .get(last_elem_index)
            .unwrap();

    seq.get_mut(i).unwrap().push(new_val);
}

fn get_row_below(cur: &Vec<i32>) -> Vec<i32> {
    let mut prev = cur.get(0).unwrap();
    let mut i = 1;

    let mut row_below = Vec::new();
    while i < cur.len() {
        let num = cur.get(i).unwrap();
        row_below.push(num - prev);
        prev = num;

        i += 1;
    }

    row_below
}

fn parse_input() -> Vec<Vec<i32>> {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut sequences = Vec::new();
    for line in contents.lines() {
        let mut cur_sequence = Vec::new();
        for str in line.split(" ") {
            let num = str.parse::<i32>().unwrap();
            cur_sequence.push(num);
        }
        sequences.push(cur_sequence);
    }

    sequences
}

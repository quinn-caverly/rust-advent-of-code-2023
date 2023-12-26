use std::{fs, iter::zip};

fn main() {
    let (time, distance) = parse_input();

    println!("{}, {}", time, distance);

    let val = calculate_ways(time, distance);

    println!("{}", val);
}

fn parse_input() -> (u64, u64){
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut lines = contents.lines();

    let time_str = lines.next().unwrap();
    let times = time_str.strip_prefix("Time:").unwrap();
    let distances_str = lines.next().unwrap();
    let distances = distances_str.strip_prefix("Distance:").unwrap();

    let mut final_time_str: String = "".to_string();
    for ch in times.chars() {
        match ch.to_digit(10) {
            Some(_) => final_time_str = final_time_str + &ch.to_string(),
            None => (),
        }
    }

    let mut final_distance_str: String = "".to_string();
    for ch in distances.chars() {
        match ch.to_digit(10) {
            Some(_) => final_distance_str = final_distance_str + &ch.to_string(),
            None => (),
        }
    }

    let time = final_time_str.parse::<u64>().unwrap();
    let distance = final_distance_str.parse::<u64>().unwrap();

    (time, distance)
}

fn calculate_ways(time: u64, distance: u64) -> u32 {
    let mut ways = 0;

    let mut time_held = 0;
    loop {
        if time_held > time {
            break;
        }

        let cur = calculate_distance(time_held, time);

        if cur > distance {
            ways += 1;
        }

        time_held += 1
    }

    ways
}

fn calculate_distance(time_held: u64, total_time: u64) -> u64 {
    // time_held is also the velocity
    let velocity = time_held;
    velocity * (total_time-time_held)
}

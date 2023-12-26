use std::{fs, iter::zip};

fn main() {
    let (time_vec, distances_vec) = parse_input();

    let mut vals = Vec::new();
    for (time, distance) in zip(time_vec, distances_vec) {
        vals.push(calculate_ways(time, distance));
    }

    let mut ans = if vals.len() == 0 {
        0
    } else {
        1
    };

    for val in vals {
        ans *= val;
    }

    println!("{}", ans);
}

fn parse_input() -> (Vec<u32>, Vec<u32>){
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut lines = contents.lines();

    let time_str = lines.next().unwrap();
    let times = time_str.strip_prefix("Time:").unwrap();
    let distances_str = lines.next().unwrap();
    let distances = distances_str.strip_prefix("Distance:").unwrap();

    let mut time_vec: Vec<u32> = Vec::new();
    for time in times.split(" ") {
        match time.parse::<u32>() {
            Ok(x) => time_vec.push(x),
            Err(_) => (),
        }
    }

    let mut distance_vec: Vec<u32> = Vec::new();
    for distance in distances.split(" ") {
        match distance.parse::<u32>() {
            Ok(x) => distance_vec.push(x),
            Err(_) => (),
        }
    }

    (time_vec, distance_vec)
}

fn calculate_ways(time: u32, distance: u32) -> u32 {
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

fn calculate_distance(time_held: u32, total_time: u32) -> u32 {
    // time_held is also the velocity
    let velocity = time_held;
    velocity * (total_time-time_held)
}

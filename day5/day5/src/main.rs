use std::{fs, str::Lines};

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();
    let mut lines = contents.lines();

    let seeds = parse_seeds(&mut lines);
    lines.next().unwrap();

    let maps = parse_maps(&mut lines);

    let mut final_seeds: Vec<u64> = Vec::new();
    let total_seeds = seeds.len();
    let mut done_seeds = 0;
    for seed in seeds {
        final_seeds.push(calculate_final_val(seed, &maps));
        done_seeds += 1;

        if done_seeds % 100000 == 0 {
            println!("{}", done_seeds/total_seeds);
        }
    }

    let minimum = final_seeds.iter().min().unwrap();
    println!("{}", minimum);
}

fn calculate_final_val(seed: u64, maps: &Vec<Vec<Range>>) -> u64 {
    let mut cur = seed;

    for map in maps {
        cur = find_corresponding_dest(cur, map);
    }
    cur
}

fn find_corresponding_dest(source: u64, map: &Vec<Range>) -> u64 {
    let mut dests: Vec<u64> = Vec::new();
    for range in map {
        if source >= range.start && source <= range.stop {
            dests.push(((source as i128) + range.add) as u64);
        }
    }

    if dests.len() == 0 {
        dests.push(source);
    }

    *dests.iter().min().unwrap()
}

// for a number between start and stop inclusive, sum with add to get the destination
// start and stop correspond to the source, add to calculate the destination
struct Range {
    start: u64,
    stop: u64,
    add: i128,
}

// the name of the map is irrelevant, just their relative ordering is significant
fn parse_maps(lines: &mut Lines<'_>) -> Vec<Vec<Range>> {
    let mut maps = Vec::new();

    let mut cont = true;
    while cont {
        let (cur, cur_cont) = parse_map(lines);
        cont = cur_cont;

        maps.push(cur);
    }

    maps
}

fn parse_map(lines: &mut Lines<'_>) -> (Vec<Range>, bool) {
    let mut map = Vec::new();

    // this will be the label
    lines.next().unwrap();

    let cont: bool;
    loop {
        let cur = match lines.next() {
            Some(line) => line,
            None => {
                cont = false;
                break;
            }
        };

        if cur == "" {
            cont = true;
            break;
        }

        let mut nums = cur.split(" ");
        let (dest, source, length) = (
            nums.next().unwrap().parse::<u64>().unwrap(),
            nums.next().unwrap().parse::<u64>().unwrap(),
            nums.next().unwrap().parse::<u64>().unwrap(),
        );
        let start = source;
        let stop = source + length - 1;
        let add = (dest as i128) - (source as i128);

        map.push(Range { start, stop, add });
    }

    (map, cont)
}

fn parse_seeds(lines: &mut Lines<'_>) -> Vec<u64> {
    let mut seeds = Vec::new();

    let mut seed_line = lines.next().unwrap();
    seed_line = &seed_line[7..];

    let mut on_start: bool = true;
    let mut start: u64 = 0;
    let mut add: u64;
    for seed in seed_line.split(" ") {
        let seed_num = seed.parse::<u64>().unwrap();

        if on_start {
            start = seed_num;
            on_start = false;
        } else {
            add = seed_num;
            on_start = true;

            for i in start..start+add {
                seeds.push(i);
            }
        }
        seeds.push(seed_num);
    }

    seeds
}

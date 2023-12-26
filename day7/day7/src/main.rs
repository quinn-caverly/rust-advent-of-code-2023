use std::{collections::HashMap, fs};

fn main() {
    let mut hands = parse_input();

    hands.sort_by(compare_hands);

    for hand in &hands {
        println!("{:?}, {}", hand.chs, hand.bid);
    }

    let mut ans = 0;
    let mut cur_rank = 1;
    for hand in hands {
        ans += cur_rank * hand.bid;
        cur_rank += 1;
    }

    println!("{}", ans);
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let mut value_map: HashMap<char, u32> = HashMap::new();
    let mut cur = 2;
    while cur < 10 {
        value_map.insert((cur as u8 + b'0') as char, cur);
        cur += 1;
    }
    value_map.insert('T', 10);
    value_map.insert('J', 11);
    value_map.insert('Q', 12);
    value_map.insert('K', 13);
    value_map.insert('A', 14);

    let a_rank = get_type_rank(&a);
    let b_rank = get_type_rank(&b);

    if a_rank < b_rank {
        return std::cmp::Ordering::Less;
    } else if a_rank > b_rank {
        return std::cmp::Ordering::Greater;
    } else {
        // we go card by card from the front until the cards aren't equal
        let mut index = 0;
        while index < 5 {
            let cur_a = a.chs.get(index).unwrap();
            let cur_b = b.chs.get(index).unwrap();

            if cur_a != cur_b {
                let a_val = value_map.get(cur_a).unwrap();
                let b_val = value_map.get(cur_b).unwrap();

                if a_val < b_val {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            }

            index += 1;
        }

        panic!("Did not expect hands to be identical");
    }
}

fn get_type_rank(hand: &Hand) -> u32 {
    // 5 of a kind is highest with val 7
    let mut cur_val = 7;
    if check_if_five_of_a_kind(hand) {
        return cur_val;
    }

    cur_val -= 1;
    if check_if_four_of_a_kind(hand) {
        return cur_val;
    }

    cur_val -= 1;
    if check_if_full_house(hand) {
        return cur_val;
    }

    cur_val -= 1;
    if check_if_three_of_a_kind(hand) {
        return cur_val;
    }

    cur_val -= 1;
    if check_if_two_pair(hand) {
        return cur_val;
    }

    cur_val -= 1;
    if check_if_one_pair(hand) {
        return cur_val;
    }

    cur_val -= 1;
    return cur_val;
}

fn check_if_one_pair(hand: &Hand) -> bool {
    // to be at least one pair, cur must be equal to prev at least once
    let mut sorted_chs = hand.chs.clone();
    sorted_chs.sort();

    let mut prev = sorted_chs.get(0).unwrap();
    let mut count = 0;
    let mut cur = 1;
    while cur < 5 {
        if sorted_chs.get(cur).unwrap() == prev {
            count += 1;
        }

        prev = sorted_chs.get(cur).unwrap();
        cur += 1;
    }

    if count > 0 {
        return true;
    } else {
        return false;
    }
}

fn check_if_two_pair(hand: &Hand) -> bool {
    // CAABB or AACBB or AABBC
    // so note the pair will always be 1st index and 3rd index
    let mut sorted_chs = hand.chs.clone();
    sorted_chs.sort();

    let first = &sorted_chs.get(1).unwrap();
    let second = &sorted_chs.get(3).unwrap();
    let mut first_count = 0;
    let mut second_count = 0;

    for ch in &sorted_chs {
        if ch == *first {
            first_count += 1;
        } else if ch == *second {
            second_count += 1;
        }
    }

    if (first_count == 2) && (second_count == 2) {
        return true;
    } else {
        return false;
    }
}

fn check_if_three_of_a_kind(hand: &Hand) -> bool {
    let mut sorted_chs = hand.chs.clone();
    sorted_chs.sort();

    // the three are either in the beginning, in the middle, at the end
    let first = sorted_chs.get(0).unwrap();
    let mut first_count = 0;
    let second = sorted_chs.get(1).unwrap();
    let mut second_count = 0;
    let last = sorted_chs.get(4).unwrap();
    let mut last_count = 0;

    for ch in &sorted_chs {
        if ch == first {
            first_count += 1;
        } else if ch == second {
            second_count += 1;
        } else if ch == last {
            last_count += 1;
        }
    }

    if (first_count == 3) || (second_count == 3) || (last_count == 3) {
        return true;
    } else {
        return false;
    }
}

fn check_if_full_house(hand: &Hand) -> bool {
    let mut sorted_chs = hand.chs.clone();
    sorted_chs.sort();

    // in a full house it can only not be the same as the previous a single time
    let mut differences = 0;

    let mut prev = sorted_chs.get(0).unwrap();
    let mut cur = 1;
    while cur < 5 {
        if prev != sorted_chs.get(cur).unwrap() {
            differences += 1;
        }
        prev = sorted_chs.get(cur).unwrap();

        cur += 1;
    }

    return if differences <= 1 { true } else { false };
}

fn check_if_four_of_a_kind(hand: &Hand) -> bool {
    let mut sorted_chs = hand.chs.clone();
    sorted_chs.sort();

    let mut prev = sorted_chs.get(0).unwrap();
    let mut cur = 1;
    while cur < 4 {
        if prev != sorted_chs.get(cur).unwrap() {
            break;
        }
        prev = sorted_chs.get(cur).unwrap();

        if cur == 3 {
            return true;
        }
        cur += 1;
    }

    sorted_chs.reverse();

    let mut prev = sorted_chs.get(0).unwrap();
    let mut cur = 1;
    while cur < 4 {
        if prev != sorted_chs.get(cur).unwrap() {
            break;
        }
        prev = sorted_chs.get(cur).unwrap();

        if cur == 3 {
            return true;
        }
        cur += 1;
    }

    return false;
}

fn check_if_five_of_a_kind(hand: &Hand) -> bool {
    let mut prev = hand.chs.get(0).unwrap();
    let mut cur = 1;
    while cur < 5 {
        if prev != hand.chs.get(cur).unwrap() {
            break;
        }
        prev = hand.chs.get(cur).unwrap();

        if cur == 4 {
            return true;
        }

        cur += 1;
    }
    return false;
}

struct Hand {
    chs: Vec<char>,
    bid: u32,
}

fn parse_input() -> Vec<Hand> {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut hands = Vec::new();

    for line in contents.lines() {
        let mut portions = line.split(" ");
        let ch_portion = portions.next().unwrap();
        let bid_portion = portions.next().unwrap();

        let mut chs: Vec<char> = Vec::new();
        for ch in ch_portion.chars() {
            chs.push(ch);
        }

        let bid = bid_portion.parse::<u32>().unwrap();

        hands.push(Hand { chs, bid });
    }
    hands
}

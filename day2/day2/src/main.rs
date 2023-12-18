use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").unwrap();

    let mut sum = 0;
    let mut game = 0;

    'line: for line in contents.lines() {
        game += 1;

        let mut portions = line.split(": ");
        let pertinent_portion = portions.nth(1).unwrap();

        let mut min_reds = 0;
        let mut min_blues = 0;
        let mut min_greens = 0;

        let hands = pertinent_portion.split("; ");
        for hand in hands {
            let cats = parse_hand_to_category(hand);

            for cat in cats {
                match cat.color {
                    Color::Blue => {
                        if cat.count > min_blues {
                            min_blues = cat.count;
                        }
                    }
                    Color::Green => {
                        if cat.count > min_greens {
                            min_greens = cat.count;
                        }
                    }
                    Color::Red => {
                        if cat.count > min_reds {
                            min_reds = cat.count;
                        }
                    }
                }
            }
        }
        sum += min_reds*min_greens*min_blues;
    }

    println!("{}", sum);
}

fn parse_hand_to_category(hand: &str) -> Vec<Category> {
    let mut categories = Vec::new();

    let cats = hand.split(", ");
    // each will be of the type {num} {color}
    for cat in cats {
        let mut portions = cat.split(" ");

        let count = portions.next().unwrap().parse::<i32>().unwrap();
        let color = match portions.next().unwrap() {
            "blue" => Color::Blue,
            "green" => Color::Green,
            "red" => Color::Red,
            _ => panic!("Did not expect color not red, blue, or green"),
        };

        categories.push(Category { color, count });
    }

    categories
}

struct Category {
    color: Color,
    count: i32,
}

enum Color {
    Blue,
    Green,
    Red,
}

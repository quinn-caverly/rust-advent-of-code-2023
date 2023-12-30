const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let hands = line
                    .strip_prefix(&format!("Game {}: ", i + 1))
                    .expect(&format!("Panicked on: {}: {}", i + 1, line))
                    .split("; ");

                for hand in hands {
                    let cards = hand.split(", ");
                    for card in cards {
                        let mut portions = card.split(" ");

                        let count = portions.next().unwrap().parse::<u32>().unwrap();
                        let color = portions.next().unwrap();
                        if (color == "red" && count > RED)
                            || (color == "blue" && count > BLUE)
                            || (color == "green" && count > GREEN)
                        {
                            return 0;
                        }
                    }
                }

                return i + 1;
            })
            .sum::<usize>()
    )
}

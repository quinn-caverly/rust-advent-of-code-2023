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

                let (mut red, mut green, mut blue) = (0, 0, 0);
                for hand in hands {
                    let cards = hand.split(", ");
                    for card in cards {
                        let mut portions = card.split(" ");

                        let count = portions.next().unwrap().parse::<u32>().unwrap();
                        let color = portions.next().unwrap();

                        if color == "red" && count > red {
                            red = count;
                        }
                        if color == "blue" && count > blue {
                            blue = count;
                        }
                        if color == "green" && count > green {
                            green = count;
                        }
                    }
                }
                red * green * blue
            })
            .sum::<u32>()
    )
}

fn main() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|line| {
                ((line.bytes().find(|b| b.is_ascii_digit())).unwrap() - b'0') as u32 * 10
                    + ((line.bytes().rev().find(|b| b.is_ascii_digit())).unwrap()
                    - b'0') as u32
            })
            .sum::<u32>()
    );
}

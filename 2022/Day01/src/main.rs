use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|amount| amount.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories.sort_by(|a, b| b.cmp(a));
    let (p1, p2): (u32, u32) = (calories[0], calories[0..=2].iter().sum());

    println!("Part 1: {p1}\nPart 2: {p2}");
}

use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            (
                left.trim().parse::<i32>().expect("Unable to read number"),
                right.trim().parse::<i32>().expect("Unable to read number"),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    let mut entries = HashMap::new();
    for &r in &right {
        *entries.entry(r).or_insert(0) += 1;
    }

    let p1: i32 = left.iter().zip(&right).map(|(x, y)| (x - y).abs()).sum();
    let p2: i32 = left
        .iter()
        .map(|&r| entries.get(&r).unwrap_or(&0) * r)
        .sum();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

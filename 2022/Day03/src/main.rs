use itertools::Itertools;
use std::fs;

const LOWER_A_POS: u8 = b'a';
const UPPER_A_POS: u8 = b'A';
const LOWER_Z_POS: u8 = b'z';
const UPPER_Z_POS: u8 = b'Z';

fn priority(num: u128) -> u128 {
    let num_zeroes: u8 = num.trailing_zeros() as u8;
    match num_zeroes {
        UPPER_A_POS..=UPPER_Z_POS => (num_zeroes - 38) as u128,
        LOWER_A_POS..=LOWER_Z_POS => (num_zeroes - 96) as u128,
        _ => unreachable!(),
    }
}

fn mask(s: &str) -> u128 {
    s.bytes().fold(0, |acc, x| acc | (1 << x))
}

fn main() {
    let input = fs::read_to_string("input").expect("Error opening input");

    let p1 = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            priority(mask(a) & mask(b))
        })
        .sum::<u128>();

    let p2 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let group_masks = group.map(mask).collect::<Vec<_>>();
            priority(
                group_masks
                    .into_iter()
                    .reduce(|acc, x| acc & x)
                    .unwrap_or(0),
            )
        })
        .sum::<u128>();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

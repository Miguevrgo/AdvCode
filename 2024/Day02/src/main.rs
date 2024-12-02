use std::fs;

fn part1() {
    let mut counter: u32 = 0;
    let contents = fs::read_to_string("./input").unwrap();
    let data: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().expect("Error"))
                .collect()
        })
        .collect();

    for row in data.iter() {
        if is_monotonic(row) && is_short(row) {
            counter += 1;
        }
    }

    println!("{}", counter);
}

fn is_short(row: &Vec<u32>) -> bool {
    row.windows(2)
        .all(|w| (w[0] as i32 - w[1] as i32).abs() <= 3)
}

fn is_monotonic(row: &Vec<u32>) -> bool {
    let is_increasing = row.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = row.windows(2).all(|w| w[0] > w[1]);
    is_increasing || is_decreasing
}

fn can_be_made_safe(row: &Vec<u32>) -> bool {
    for i in 0..row.len() {
        let mut temp_row = row.clone();
        temp_row.remove(i);
        if is_monotonic(&temp_row) && is_short(&temp_row) {
            return true;
        }
    }
    false
}

fn part2() {
    let mut counter: u32 = 0;
    let contents = fs::read_to_string("./input").unwrap();
    let data: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().expect("Error"))
                .collect()
        })
        .collect();

    for row in data.iter() {
        if can_be_made_safe(row) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn main() {
    part1();
    part2();
}

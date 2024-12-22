use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("File not found");
    let mut p1 = 0;
    for line in input.lines() {
        let mut num: i64 = line.parse().unwrap();
        for _ in 0..2000 {
            num = (num ^ (num * 64)) % 16777216;
            num = (num ^ (num / 32)) % 16777216;
            num = (num ^ (num * 2048)) % 16777216;
        }
        p1 += num;
    }

    println!("Part 1: {p1}");
}

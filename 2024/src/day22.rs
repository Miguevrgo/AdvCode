use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../input/day22.txt");
    let (mut p1, mut p2): (i64, HashMap<i64, i64>) = (0, HashMap::new());
    let mut seen = HashSet::new();
    for line in input.lines() {
        let mut last_digit = vec![0; 2000];
        let mut num: i64 = line.parse().unwrap();
        for val in last_digit.iter_mut().take(2000) {
            num = (num ^ (num * 64)) % 16777216;
            num = (num ^ (num / 32)) % 16777216;
            num = (num ^ (num * 2048)) % 16777216;
            *val = num % 10;
        }
        p1 += num;

        seen.clear();
        for i in 0..1996 {
            let k = (last_digit[i + 1] - last_digit[i])
                + (last_digit[i + 2] - last_digit[i + 1]) * 19
                + (last_digit[i + 3] - last_digit[i + 2]) * 361
                + (last_digit[i + 4] - last_digit[i + 3]) * 6859;
            if seen.insert(k) {
                *p2.entry(k).or_default() += last_digit[i + 4];
            }
        }
    }

    println!("Part 1: {p1}\nPart 2: {}", *p2.values().max().unwrap() + 4);
}

use std::collections::HashMap;

fn update(mut stones: HashMap<i64, usize>, iterations: usize) -> HashMap<i64, usize> {
    for _ in 0..iterations {
        let mut updates = vec![];

        for (&stone, &count) in &stones {
            match stone {
                0 => updates.push((1, count)),
                _ => {
                    let digits = stone.ilog10() + 1;
                    if digits % 2 == 0 {
                        let div = 10i64.pow(digits / 2);
                        updates.push((stone % div, count));
                        updates.push((stone / div, count));
                    } else {
                        updates.push((stone * 2024, count));
                    }
                }
            }
        }

        stones.clear();
        for (key, value) in updates {
            *stones.entry(key).or_default() += value;
        }
    }
    stones
}

pub fn run() {
    let input = include_str!("../input/day11.txt");
    let stones = input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .map(|num| (num, 1))
        .collect::<HashMap<_, _>>();

    let p1_stones = update(stones.clone(), 25);
    let p1: usize = p1_stones.values().sum();

    let p2: usize = update(p1_stones, 50).values().sum();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

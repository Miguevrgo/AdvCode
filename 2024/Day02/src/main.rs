use std::fs;

fn is_safe(x: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..x.len() - 1 {
        if x[i] < x[i + 1] {
            increasing = false;
        }
        if x[i] > x[i + 1] {
            decreasing = false;
        }
        if !(1..=3).contains(&x[i].abs_diff(x[i + 1])) {
            return false;
        }
    }

    increasing || decreasing
}

fn can_be_safe(x: &[i32]) -> bool {
    (0..x.len()).any(|i| {
        let mut y = x.to_vec();
        y.remove(i);
        is_safe(&y)
    })
}

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let (mut p1, mut p2) = (0, 0);

    for line in input.lines() {
        let x = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&x) {
            p1 += 1;
        }
        if can_be_safe(&x) {
            p2 += 1;
        }
    }

    println!("Part 1: {p1}\nPart 2: {p2}");
}

use std::fs;

/// (x, y) = a(v1, v2) + b(w1, w2)
/// Solve for a and b
fn solve(v1: i64, v2: i64, w1: i64, w2: i64, x: i64, y: i64) -> i64 {
    let a = (x * w2 - y * w1) / (v1 * w2 - v2 * w1);
    let b = (x - v1 * a) / w1;

    // If stable, add [unlikely]
    if (x, y) != (v1 * a + w1 * b, v2 * a + w2 * b) {
        return 0;
    }

    3 * a + b
}

fn main() {
    let input = fs::read_to_string("input").expect("No file input found");
    let (mut p1, mut p2) = (0, 0);
    for line in input.split("\n\n") {
        let numbers: Vec<i64> = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|num| !num.is_empty())
            .map(|num| num.parse().unwrap())
            .collect();
        if let [v1, v2, w1, w2, x, y] = numbers[..] {
            p1 += solve(v1, v2, w1, w2, x, y);
            p2 += solve(v1, v2, w1, w2, x + 10000000000000, y + 10000000000000);
        } else {
            panic!("Incorrect arguments")
        }
    }

    println!("Part 1: {p1}\nPart 2: {p2}");
}

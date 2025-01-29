use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error opening input");
    let pairs = input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (a, b) = left.split_once('-').unwrap();
            let (c, d) = right.split_once('-').unwrap();
            Some((
                a.parse().ok()?,
                b.parse().ok()?,
                c.parse().ok()?,
                d.parse().ok()?,
            ))
        })
        .collect::<Vec<(usize, _, _, _)>>();

    let p1 = pairs
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count();
    let p2 = pairs.iter().filter(|(a, b, c, d)| a <= d && c <= b).count();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

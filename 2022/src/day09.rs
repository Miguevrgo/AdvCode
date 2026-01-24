use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day09.txt");
    let mut visited = HashSet::with_capacity(4096);
    visited.insert((0, 0));
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);

    let instructions: Vec<(char, i32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_at(1);
            (
                left.chars().next().unwrap(),
                right.trim().parse::<i32>().unwrap(),
            )
        })
        .collect();

    for (dir, amount) in instructions {
        for _ in 0..amount {
            match dir {
                'U' => head_pos.1 += 1,
                'D' => head_pos.1 -= 1,
                'R' => head_pos.0 += 1,
                'L' => head_pos.0 -= 1,
                _ => unreachable!(),
            }
            if (tail_pos.0).abs_diff(head_pos.0) | (tail_pos.1).abs_diff(head_pos.1) > 1 {
                tail_pos.0 += (head_pos.0 - tail_pos.0).clamp(-1, 1);
                tail_pos.1 += (head_pos.1 - tail_pos.1).clamp(-1, 1);
                visited.insert((tail_pos.0, tail_pos.1));
            }
        }
    }

    println!("Part 1: {}", visited.len())
}

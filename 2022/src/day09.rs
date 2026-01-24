use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day09.txt");
    let mut visited_p1 = HashSet::with_capacity(4096);
    let mut visited_p2 = HashSet::with_capacity(2048);

    visited_p1.insert((0, 0));
    visited_p2.insert((0, 0));
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: [(i32, i32); 9] = [(0, 0); 9];

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

            if (tail_pos[0].0).abs_diff(head_pos.0) | (tail_pos[0].1).abs_diff(head_pos.1) > 1 {
                tail_pos[0].0 += (head_pos.0 - tail_pos[0].0).signum();
                tail_pos[0].1 += (head_pos.1 - tail_pos[0].1).signum();
            }

            for i in 1..9 {
                let prev = tail_pos[i - 1];
                if (tail_pos[i].0).abs_diff(prev.0) | (tail_pos[i].1).abs_diff(prev.1) > 1 {
                    tail_pos[i].0 += (prev.0 - tail_pos[i].0).signum();
                    tail_pos[i].1 += (prev.1 - tail_pos[i].1).signum();
                }
            }

            visited_p1.insert(tail_pos[0]);
            visited_p2.insert(tail_pos[8]);
        }
    }

    println!("Part 1: {} Part 2: {}", visited_p1.len(), visited_p2.len());
}

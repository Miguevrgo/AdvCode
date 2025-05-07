use std::fs;

fn part1(mut stacks: Vec<Vec<char>>, moves: &[[usize; 3]]) -> String {
    for &[amount, from, to] in moves {
        for _ in 0..amount {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c);
            }
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &[[usize; 3]]) -> String {
    for &[amount, from, to] in moves {
        if from == to {
            continue;
        }

        let (a, b) = if from < to {
            let (left, right) = stacks.split_at_mut(to);
            (&mut left[from], &mut right[0])
        } else {
            let (left, right) = stacks.split_at_mut(from);
            (&mut right[0], &mut left[to])
        };

        let len = a.len();
        let drained = a.drain(len - amount..);
        b.extend(drained);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (start, rest) = input.split_once("\n\n").unwrap();

    let lines: Vec<_> = start.lines().collect();
    let width = (lines[0].len() + 1) / 4;

    let mut original_stacks = vec![Vec::new(); width];
    for row in lines.iter().rev().skip(1) {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                original_stacks[i].push(c);
            }
        }
    }

    let moves = rest
        .lines()
        .map(|l| {
            let nums: Vec<usize> = l
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            [nums[0], nums[1] - 1, nums[2] - 1]
        })
        .collect::<Vec<_>>();

    let p1 = part1(original_stacks.clone(), &moves);
    let p2 = part2(original_stacks, &moves);

    println!("Part 1: {p1}\nPart 2: {p2}");
}

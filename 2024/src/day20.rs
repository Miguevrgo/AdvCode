use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input = include_str!("../input/day20.txt");
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start, end) =
        maze.iter()
            .enumerate()
            .fold(((0, 0), (0, 0)), |(mut s, mut e), (r, row)| {
                row.iter().enumerate().for_each(|(c, &cell)| match cell {
                    'S' => s = (r as i32, c as i32),
                    'E' => e = (r as i32, c as i32),
                    _ => {}
                });
                (s, e)
            });

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut distances = HashMap::new();
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((row, col, dist)) = queue.pop_back() {
        if distances.contains_key(&(row, col)) {
            continue;
        }
        distances.insert((row, col), dist);

        if (row, col) == end {
            continue;
        }

        for (dr, dc) in directions {
            let (new_row, new_col) = (row + dr, col + dc);
            if new_row >= 0
                && new_row < maze.len() as i32
                && new_col >= 0
                && new_col < maze[0].len() as i32
                && maze[new_row as usize][new_col as usize] != '#'
            {
                queue.push_front((new_row, new_col, dist + 1));
            }
        }
    }

    let (mut p1, mut p2) = (0, 0);

    let distances_vec: Vec<_> = distances.into_iter().collect();
    distances_vec
        .iter()
        .enumerate()
        .for_each(|(i, &((r1, c1), n1))| {
            distances_vec
                .iter()
                .skip(i + 1)
                .for_each(|&((r2, c2), n2)| {
                    let d = r1.abs_diff(r2) + c1.abs_diff(c2);
                    if d <= 20 && n2.abs_diff(n1) >= d + 100 {
                        if d <= 2 {
                            p1 += 1;
                        }
                        p2 += 1;
                    }
                });
        });

    println!("Part 1: {p1}\nPart 2: {p2}");
}

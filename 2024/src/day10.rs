use std::collections::{HashSet, VecDeque};

fn num_routes(map: &[Vec<u32>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut routes = Vec::new();
    let mut queue = VecDeque::from([(row, col)]);

    while let Some((r, c)) = queue.pop_front() {
        if map[r][c] == 9 {
            routes.push((r, c));
        } else {
            for (new_row, new_col) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                if *map
                    .get(new_row)
                    .and_then(|row| row.get(new_col))
                    .unwrap_or(&0)
                    == map[r][c] + 1
                {
                    queue.push_back((new_row, new_col));
                }
            }
        }
    }

    routes
}

pub fn run() {
    let input = include_str!("../input/day10.txt");
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect();

    let (mut p1, mut p2) = (0, 0);
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                let routes = num_routes(&map, row, col);
                p1 += routes.iter().collect::<HashSet<_>>().len();
                p2 += routes.len();
            }
        }
    }

    println!("Part 1: {p1}\nPart 2: {p2}")
}

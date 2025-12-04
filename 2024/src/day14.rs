use std::collections::HashSet;

const TILE_WIDTH: i32 = 101;
const TILE_HEIGHT: i32 = 103;

fn safety_factor(robots: &[(i64, i64, i64, i64)]) -> usize {
    let mut sectors = [0; 4];
    robots.iter().for_each(|&(r, c, _, _)| {
        if r != TILE_WIDTH as i64 / 2 && c != TILE_HEIGHT as i64 / 2 {
            let a = (r < TILE_WIDTH as i64 / 2) as usize;
            let b = (c < TILE_HEIGHT as i64 / 2) as usize;
            sectors[a * 2 + b] += 1;
        }
    });
    sectors.iter().product()
}

pub fn run() {
    let input = include_str!("../input/day14.txt");

    let mut robots: Vec<i32> = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|num| num.parse::<i32>().ok()) // Filtra y parsea a i32
        .collect();

    for i in 1.. {
        robots.chunks_exact_mut(4).for_each(|chunk| {
            if let [x1, x2, v1, v2] = chunk {
                *x1 = (*x1 + *v1).rem_euclid(TILE_WIDTH);
                *x2 = (*x2 + *v2).rem_euclid(TILE_HEIGHT);
            }
        });

        if i == 100 {
            let robots_tuples: Vec<_> = robots
                .chunks_exact(4)
                .map(|chunk| {
                    (
                        chunk[0] as i64,
                        chunk[1] as i64,
                        chunk[2] as i64,
                        chunk[3] as i64,
                    )
                })
                .collect();
            let p1 = safety_factor(&robots_tuples);
            println!("Part 1: {p1}");
        }

        let mut positions = HashSet::new();
        let all_unique = robots
            .chunks_exact(4)
            .map(|chunk| (chunk[0], chunk[1]))
            .all(|pos| positions.insert(pos));

        if all_unique {
            let p2 = i;
            println!("Part 2: {p2}");
            break;
        }
    }
}

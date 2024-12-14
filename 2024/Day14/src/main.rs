use std::{collections::HashSet, fs};

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

fn print_grids(all_robots: &Vec<(Vec<(i32, i32)>, usize)>) {
    let num_grids = all_robots.len();
    let num_cols = 20;
    let num_rows = (num_grids + num_cols - 1) / num_cols;

    for row in 0..num_rows {
        for y in 0..TILE_HEIGHT {
            for col in 0..num_cols {
                let grid_index = row * num_cols + col;
                if grid_index < num_grids {
                    let (robots, iteration) = &all_robots[grid_index];
                    let mut grid_line = String::new();
                    if y == 0 {
                        grid_line.push_str(&format!("{:<3}", iteration));
                        for _ in 0..(TILE_WIDTH as usize - 3) {
                            grid_line.push(' ');
                        }
                    } else {
                        for x in 0..TILE_WIDTH {
                            let mut found = false;
                            for &(rx, ry) in robots {
                                if rx == x && ry == y {
                                    grid_line.push('#');
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                grid_line.push('.');
                            }
                        }
                    }

                    print!("{:<105}  ", grid_line); // Formato ajustado
                } else {
                    print!("{:<105}  ", " ".repeat(TILE_WIDTH as usize)); // Espacios en blanco
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("File not found");

    let mut robots: Vec<i32> = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|num| num.parse().ok())
        .collect();

    let mut all_robot_positions: Vec<(Vec<(i32, i32)>, usize)> = Vec::new();

    for i in 1.. {
        robots.chunks_exact_mut(4).for_each(|chunk| {
            if let [x1, x2, v1, v2] = chunk {
                *x1 = (*x1 + *v1).rem_euclid(TILE_WIDTH);
                *x2 = (*x2 + *v2).rem_euclid(TILE_HEIGHT);
            }
        });

        let robot_positions: Vec<(i32, i32)> = robots
            .chunks_exact(4)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        all_robot_positions.push((robot_positions, i));
        if i % 20 == 0 || i == 1 {
            print_grids(&all_robot_positions);
            all_robot_positions.clear();
        }

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
    }
}

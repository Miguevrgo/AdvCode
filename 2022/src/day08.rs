use std::io::BufRead;

fn part2(grid: &[Vec<(u32, bool)>], r: usize, c: usize, size: usize) -> usize {
    let height = grid[r][c].0;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(dr, dc)| {
            let mut dist = 0;
            let mut nr = r as isize + dr;
            let mut nc = c as isize + dc;
            let limit = size as isize;

            while nr >= 0 && nc >= 0 && nr < limit && nc < limit {
                dist += 1;
                if grid[nr as usize][nc as usize].0 >= height {
                    break;
                }
                nr += dr;
                nc += dc;
            }
            dist
        })
        .product()
}

pub fn run() {
    let input = include_bytes!("../input/day08.txt");
    let mut matrix: Vec<_> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|ch| (ch.to_digit(10).unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect();

    // rows == cols so we can iterate vertically or horizontally
    // this allows to perform part1 in 4n iterations
    let size = matrix.len();

    for i in 0..size {
        let mut max_vals = [-1; 4];
        for j in 0..size {
            let coords = [
                (i, j),            // Left -> Right
                (i, size - 1 - j), // Right -> Left
                (j, i),            // Top -> Bottom
                (size - 1 - j, i), // Bottom -> Top
            ];

            for (k, &(r, c)) in coords.iter().enumerate() {
                if matrix[r][c].0 as i32 > max_vals[k] {
                    matrix[r][c].1 = true;
                    max_vals[k] = matrix[r][c].0 as i32;
                }
            }
        }
    }

    let p1 = matrix
        .iter()
        .flatten()
        .filter(|&&(_, visible)| visible)
        .count();

    let grid_ref = &matrix;
    let p2 = (1..size - 1)
        .flat_map(|r| (1..size - 1).map(move |c| part2(grid_ref, r, c, size)))
        .max()
        .unwrap_or(0);
    println!("Part 1: {p1} Part 2: {p2}")
}

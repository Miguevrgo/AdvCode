use std::collections::VecDeque;

fn get_elevation(c: u8) -> u8 {
    match c {
        b'S' => b'a',
        b'E' => b'z',
        _ => c,
    }
}

fn bfs<F>(grid: &[&[u8]], start: (usize, usize), target: u8, can_move: F) -> Option<usize>
where
    F: Fn(u8, u8) -> bool,
{
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::from([(start, 0)]);

    visited[start.0][start.1] = true;

    while let Some(((r, c), cost)) = queue.pop_front() {
        if grid[r][c] == target {
            return Some(cost);
        }

        let current_elev = get_elevation(grid[r][c]);

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] && can_move(current_elev, grid[nr][nc]) {
                    visited[nr][nc] = true;
                    queue.push_back(((nr, nc), cost + 1));
                }
            }
        }
    }

    unreachable!()
}

pub fn run() {
    let input = include_str!("../input/day12.txt");
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'S').map(|c| (r, c)))
        .unwrap();

    let end_p2 = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == b'E').map(|c| (r, c)))
        .unwrap();

    let p1 = bfs(&grid, start, b'E', |curr, next| next <= curr + 1).unwrap();
    let p2 = bfs(&grid, end_p2, b'a', |curr, next| curr <= next + 1).unwrap();
    println!("Part 1: {p1} Part 2: {p2}");
}

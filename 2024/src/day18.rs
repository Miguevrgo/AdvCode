use std::collections::VecDeque;

const NUM_ROWS: usize = 71;
const NUM_COLS: usize = 71;
const ROW_EXIT: i32 = 70;
const COL_EXIT: i32 = 70;

fn bfs(maze: &[Vec<i32>]) -> i32 {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut seen = [[false; 71]; 71];
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

    queue.push_front((0, 0, 0));

    while let Some((row, col, amount)) = queue.pop_back() {
        if (row, col) == (ROW_EXIT, COL_EXIT) {
            return amount;
        }
        if seen[row as usize][col as usize] {
            continue;
        }

        seen[row as usize][col as usize] = true;
        for (dr, dc) in directions {
            let (new_row, new_col) = (row + dr, col + dc);
            if new_row >= 0
                && new_row < NUM_ROWS as i32
                && new_col >= 0
                && new_col < NUM_COLS as i32
                && maze[new_row as usize][new_col as usize] != -1
            {
                queue.push_front((new_row, new_col, amount + 1));
            }
        }
    }

    0
}

pub fn run() {
    let input = include_str!("../input/day18.txt");
    let coords: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (
                x.trim().parse().expect("Invalid x coordinate"),
                y.trim().parse().expect("Invalid y coordinate"),
            )
        })
        .collect();

    let mut maze = vec![vec![0; NUM_COLS]; NUM_ROWS];

    for &(x, y) in &coords[..1024] {
        maze[y][x] = -1;
    }
    let part1_result = bfs(&maze);
    println!("Part 1: {}", part1_result);

    let mut low = 0;
    let mut high = coords.len();

    while low < high {
        let mid = (low + high) / 2;

        for &(x, y) in &coords[..=mid] {
            maze[y][x] = -1;
        }

        if bfs(&maze) == 0 {
            high = mid;
        } else {
            low = mid + 1;
        }

        for &(x, y) in &coords[..=mid] {
            maze[y][x] = 0;
        }
    }

    let (x, y) = coords[low];
    println!("Part 2: ({},{})", x, y);
}

use std::fs;

const NUM_ROWS: usize = 70;
const NUM_COLS: usize = 70;

fn main() {
    let input = fs::read_to_string("input").expect("Input file not found");
    let coords: Vec<(_, _)> = input
        .split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            (
                chunk[0]
                    .parse::<usize>()
                    .expect("Unable to parse x coordinate"),
                chunk[1]
                    .parse::<usize>()
                    .expect("Unable to parse y coordinate"),
            )
        })
        .collect();

    let mut maze: Vec<Vec<i32>> = vec![vec![0; NUM_ROWS]; NUM_COLS];

    for (x, y) in coords {
        maze[x][y] = -1;
    }
}

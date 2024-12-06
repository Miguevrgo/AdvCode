use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let directions = [
        ('>', (0, 1)),  // Right
        ('v', (1, 0)),  // Bottom
        ('<', (0, -1)), // Left
        ('^', (-1, 0)), // Top
    ];

    let (mut row, mut col, mut dir) = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find_map(|(i, j, c)| {
            directions
                .iter()
                .find(|&&(ch, _)| ch == c)
                .map(|&(_, dir)| (i as i32, j as i32, dir))
        })
        .expect("Character should be found");

    while row >= 0 && row < map.len() as i32 && col >= 0 && col < map[row as usize].len() as i32 {
        let new_row = row + dir.0;
        let new_col = col + dir.1;

        if !(0..map.len() as i32).contains(&new_row) || !(0..map[0].len() as i32).contains(&new_col)
        {
            break;
        }

        if map[new_row as usize][new_col as usize] != '#' {
            row = new_row;
            col = new_col;
            map[new_row as usize][new_col as usize] = 'X';
        } else {
            let current_dir_index = directions.iter().position(|&(_, d)| d == dir).unwrap();
            dir = directions[(current_dir_index + 1) % directions.len()].1;
        }
    }

    let (p1, p2) = (
        map.iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == 'X')
            .count()
            + 1,
        0,
    );
    println!("Part 1: {p1}\nPart 2: {p2}");
}

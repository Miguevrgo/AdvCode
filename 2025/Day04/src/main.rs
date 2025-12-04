const PAPER: u8 = 64;
const DOT: u8 = 46;

fn count_surrounding(diagram: &[Vec<u8>], pos: (usize, usize)) -> u32 {
    let mut count: u32 = 0;

    for &dr in [-1, 0, 1].iter() {
        let nr = pos.0 as i32 + dr;
        for &dc in [-1, 0, 1].iter() {
            let nc = pos.1 as i32 + dc;
            if nr >= 0
                && nc >= 0
                && nr < diagram.len() as i32
                && nc < diagram[0].len() as i32
                && diagram[nr as usize][nc as usize] == PAPER
            {
                count += 1;
            }
        }
    }

    count - 1 // Self
}

fn main() {
    let input = std::fs::read_to_string("input").expect("File not found");
    let diagram: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    let p1: u32 = diagram.iter().enumerate().fold(0, |acc, (line_n, line)| {
        let count = line
            .iter()
            .enumerate()
            .filter(|&(pos, &cont)| cont == PAPER && count_surrounding(&diagram, (line_n, pos)) < 4)
            .count();
        acc + count as u32
    });

    let mut p2 = 0;
    let mut grid = diagram.clone();
    loop {
        let mut updated_positions = Vec::new();
        let count2 = grid.iter().enumerate().fold(0, |acc, (line_n, line)| {
            let count = line
                .iter()
                .enumerate()
                .filter(|&(pos, &cont)| {
                    cont == PAPER && count_surrounding(&grid, (line_n, pos)) < 4
                })
                .count();

            updated_positions.extend(
                line.iter()
                    .enumerate()
                    .filter(|&(pos, &cont)| {
                        cont == PAPER && count_surrounding(&grid, (line_n, pos)) < 4
                    })
                    .map(|(pos, _)| (line_n, pos)),
            );

            acc + count
        });
        if count2 == 0 {
            break;
        }
        for (line_n, pos) in updated_positions {
            grid[line_n][pos] = DOT;
        }
        p2 += count2;
    }

    println!("Part 1: {p1}, Part 2: {p2}")
}

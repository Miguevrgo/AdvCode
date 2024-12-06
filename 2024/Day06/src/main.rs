use std::fs;

fn walk(
    m: &mut [Vec<char>],
    mut row: usize,
    mut col: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut dir = 0;
    loop {
        if seen[row][col][dir] {
            return None;
        }
        seen[row][col][dir] = true;

        let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][dir];
        let (new_row, new_col) = (row as isize + dr, col as isize + dc);

        if new_row < 0
            || new_row >= m.len() as isize
            || new_col < 0
            || new_col >= m[0].len() as isize
        {
            if !return_squares {
                return Some(Vec::new());
            }
            let mut visited = Vec::new();
            for (i, row) in m.iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    if seen[i][j].iter().any(|&b| b) {
                        visited.push((i, j));
                    }
                }
            }
            return Some(visited);
        }

        let (new_row, new_col) = (new_row as usize, new_col as usize);
        if m[new_row][new_col] == '#' {
            dir = (dir + 1) % 4;
        } else {
            (row, col) = (new_row, new_col);
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let mut m: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (sr, sc) = (0..m.len())
        .flat_map(|r| (0..m[0].len()).map(move |c| (r, c)))
        .find(|&(r, c)| m[r][c] == '^')
        .expect("Start position not found");

    let p1 = walk(&mut m, sr, sc, true).expect("Failed to find a valid path for p1");
    let p2 = p1
        .iter()
        .filter(|&&(r, c)| {
            let expected_dir = m[r][c];
            m[r][c] = '#'; // Block the cell
            let result = walk(&mut m, sr, sc, false);
            m[r][c] = expected_dir; // Restore the cell
            result.is_none()
        })
        .count();

    println!("Part 1: {}\nPart 2: {}", p1.len(), p2);
}

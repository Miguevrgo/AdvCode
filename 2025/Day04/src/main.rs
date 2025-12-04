const PAPER: u8 = b'@';

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let input = std::fs::read_to_string("input").expect("File not found");
    let diagram: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (h, w) = (diagram.len(), diagram[0].len());
    let mut padded = vec![vec![u8::MAX; w + 2]; h + 2];

    let mut changes = Vec::with_capacity(h * w);
    for r in 0..h {
        for c in 0..w {
            if diagram[r][c] != PAPER {
                continue;
            }

            let neighbours = OFFSETS
                .iter()
                .filter(|&&(dr, dc)| {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    nr >= 0
                        && nr < h as isize
                        && nc >= 0
                        && nc < w as isize
                        && diagram[nr as usize][nc as usize] == PAPER
                })
                .count();

            if neighbours < 4 {
                changes.push((r + 1, c + 1));
            } else {
                padded[r + 1][c + 1] = neighbours as u8;
            }
        }
    }

    let p1 = changes.len();
    let mut p2 = 0;

    while let Some((r, c)) = changes.pop() {
        p2 += 1;
        OFFSETS.iter().for_each(|&(dr, dc)| {
            let nr = (r as isize + dr) as usize;
            let nc = (c as isize + dc) as usize;
            padded[nr][nc] -= 1;

            if padded[nr][nc] == 4 {
                changes.push((nr, nc));
            }
        });
    }

    println!("Part 1: {p1}, Part 2: {p2}")
}

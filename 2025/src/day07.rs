use std::collections::HashMap;

fn dfs(
    row: usize,
    col: usize,
    grid: &Vec<&[u8]>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row == grid.len() - 1 {
        return 1;
    }

    if let Some(&cached) = cache.get(&(row, col)) {
        cached
    } else if grid[row + 1][col] == b'^' {
        let timelines = dfs(row + 1, col - 1, grid, cache) + dfs(row + 1, col + 1, grid, cache);
        cache.insert((row, col), timelines);
        timelines
    } else {
        let timelines = dfs(row + 1, col, grid, cache);
        cache.insert((row, col), timelines);
        timelines
    }
}

pub fn run() {
    let input = include_str!("../input/day07.txt");
    let start = (0, input.find('S').unwrap());
    let mut beams = std::collections::HashSet::new();
    beams.insert(start);

    let p1 = input.lines().map(str::as_bytes).fold(0, |acc, l| {
        let mut n_beams = Vec::new();
        let mut count = 0;

        for &(row, col) in &beams {
            if l[col] == b'^' {
                count += 1;
                n_beams.push((row + 1, col + 1));
                n_beams.push((row + 1, col - 1));
            } else {
                n_beams.push((row + 1, col));
            }
        }

        beams = n_beams.iter().cloned().collect();
        acc + count
    });

    let mut cache = std::collections::HashMap::new();
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let p2 = dfs(start.0, start.1, &grid, &mut cache);

    println!("Part 1: {p1} Part 2: {p2}");
}

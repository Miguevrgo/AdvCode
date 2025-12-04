use std::collections::{HashMap, HashSet};

fn add_node(
    grid: &[Vec<char>],
    antinodes: &mut HashSet<(usize, usize)>,
    (r1, c1): (usize, usize),
    (r2, c2): (usize, usize),
    part_one: bool,
) {
    let (dr, dc) = (r2 as isize - r1 as isize, c2 as isize - c1 as isize);
    let mut c = c2 as isize + dc;
    let mut r = r2 as isize + dr;

    if part_one {
        if (0..grid.len() as isize).contains(&r) && (0..grid[0].len() as isize).contains(&c) {
            antinodes.insert((r as usize, c as usize));
        }
    } else {
        antinodes.insert((r2, c2));
        while (0..grid.len() as isize).contains(&r) && (0..grid[0].len() as isize).contains(&c) {
            antinodes.insert((r as usize, c as usize));
            c += dc;
            r += dr;
        }
    }
}

pub fn run() {
    let input = include_str!("../input/day08.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nodes: HashMap<char, Vec<(usize, usize)>> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &ch)| (ch, i, j)))
        .filter(|&(ch, _, _)| ch != '.')
        .fold(HashMap::new(), |mut map, (ch, i, j)| {
            map.entry(ch).or_default().push((i, j));
            map
        });

    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

    for positions in nodes.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                add_node(&grid, &mut p1, positions[i], positions[j], true);
                add_node(&grid, &mut p1, positions[j], positions[i], true);
                add_node(&grid, &mut p2, positions[i], positions[j], false);
                add_node(&grid, &mut p2, positions[j], positions[i], false);
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", p1.len(), p2.len());
}

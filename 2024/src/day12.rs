use std::collections::{HashSet, VecDeque};

fn neighbours(g: &[Vec<char>], r: usize, c: usize) -> Vec<(usize, usize)> {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let rr = r.wrapping_add(dr as usize);
            let cc = c.wrapping_add(dc as usize);
            if g.get(rr).and_then(|row| row.get(cc)) == Some(&g[r][c]) {
                Some((rr, cc))
            } else {
                None
            }
        })
        .collect()
}

fn perimiter(g: &[Vec<char>], a: &HashSet<(usize, usize)>) -> usize {
    a.iter().map(|&(r, c)| 4 - neighbours(g, r, c).len()).sum()
}

fn sides(a: &HashSet<(usize, usize)>) -> usize {
    let mut s = HashSet::new();
    for &(r, c) in a {
        for &(dr, dc) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
            if a.contains(&(r.wrapping_add(dr as usize), c.wrapping_add(dc as usize))) {
                continue;
            }
            let (mut rr, mut cc) = (r, c);
            while a.contains(&(rr.wrapping_add(dc as usize), cc.wrapping_add(dr as usize)))
                && !a.contains(&(rr.wrapping_add(dr as usize), cc.wrapping_add(dc as usize)))
            {
                rr = rr.wrapping_add(dc as usize);
                cc = cc.wrapping_add(dr as usize);
            }
            s.insert((rr, cc, dr, dc));
        }
    }
    s.len()
}

fn find_shape(
    g: &[Vec<char>],
    r: usize,
    c: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut q = VecDeque::from([(r, c)]);
    let mut a = HashSet::from([(r, c)]);
    while let Some((r, c)) = q.pop_front() {
        for (rr, cc) in neighbours(g, r, c) {
            if seen.insert((rr, cc)) {
                a.insert((rr, cc));
                q.push_back((rr, cc));
            }
        }
    }
    a
}

pub fn run() {
    let input = include_str!("../input/day12.txt");
    let g = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut seen = HashSet::new();
    let (mut p1, mut p2) = (0, 0);
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if seen.contains(&(r, c)) {
                continue;
            }
            let a = find_shape(&g, r, c, &mut seen);
            p1 += a.len() * perimiter(&g, &a);
            p2 += a.len() * sides(&a);
        }
    }
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

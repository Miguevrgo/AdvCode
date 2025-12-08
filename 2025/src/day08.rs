use itertools::Itertools;

const ITERATIONS: usize = 1000;

struct Node {
    parent: usize,
    size: usize,
}

fn find(set: &mut [Node], mut x: usize) -> usize {
    while set[x].parent != x {
        let parent = set[x].parent;
        (x, set[x].parent) = (parent, set[parent].parent);
    }
    x
}

fn union(set: &mut [Node], mut x: usize, mut y: usize) -> bool {
    x = find(set, x);
    y = find(set, y);

    if x != y {
        if set[x].size < set[y].size {
            std::mem::swap(&mut x, &mut y);
        }

        set[y].parent = x;
        set[x].size += set[y].size;
        true
    } else {
        false
    }
}

fn distance(x: &(i64, i64, i64), y: &(i64, i64, i64)) -> i64 {
    (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2)
}

pub fn run() {
    let input = include_str!("../input/day08.txt");

    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut pairs = Vec::with_capacity(points.len() * points.len() / 2);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push((distance(&points[i], &points[j]), i, j));
        }
    }
    pairs.sort_unstable_by_key(|k| k.0);

    let mut nodes: Vec<Node> = (0..points.len())
        .map(|i| Node { parent: i, size: 1 })
        .collect();

    for &(_, i, j) in pairs.iter().take(ITERATIONS) {
        union(&mut nodes, i, j);
    }

    let mut circuit_sizes: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if n.parent == i { Some(n.size) } else { None })
        .collect();

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let p1: usize = circuit_sizes.iter().take(3).product();
    let mut nodes_p2: Vec<Node> = (0..points.len())
        .map(|i| Node { parent: i, size: 1 })
        .collect();

    let mut num_components = points.len();

    let mut p2 = 0;
    for &(_, i, j) in pairs.iter() {
        if union(&mut nodes_p2, i, j) {
            num_components -= 1;

            if num_components == 1 {
                p2 = points[i].0 * points[j].0;
            }
        }
    }
    println!("Part 1: {p1}, Part 2: {p2}");
}

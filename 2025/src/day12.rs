pub fn run() {
    let input = include_str!("../input/day12.txt");
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let (regions_part, sizes_part) = parts.split_last().unwrap();

    let sizes = sizes_part
        .iter()
        .map(|p| p.bytes().filter(|&b| b == b'#').count())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    for region in regions_part.lines() {
        let (x, rest) = region.split_once(": ").unwrap();
        let (a, b) = x.split_once('x').unwrap();
        let (r, c) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());

        let total_needed = rest
            .split_whitespace()
            .zip(&sizes)
            .map(|(n, size)| n.parse::<usize>().unwrap() * size)
            .sum::<usize>();

        if total_needed <= r * c {
            p1 += 1;
        }
    }
    println!("Part 1: {p1}");
}

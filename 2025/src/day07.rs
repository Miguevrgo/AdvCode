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

    println!("Part 1: {p1}");
}

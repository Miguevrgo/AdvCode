pub fn run() {
    let input = include_str!("../input/day07.txt");
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let start = lines[0].iter().position(|&ch| ch == b'S').unwrap();

    let mut beams = vec![0; width];
    let mut n_beams = vec![0; width];
    let mut p1 = 0;
    beams[start] = 1;

    lines.iter().for_each(|line| {
        for (col, &count) in beams.iter().enumerate().filter(|(_, &cnt)| cnt > 0) {
            if line[col] == b'^' {
                p1 += 1;
                n_beams[col + 1] += count;
                n_beams[col - 1] += count;
            } else {
                n_beams[col] += count;
            }
        }

        beams = n_beams.clone();
        n_beams.fill(0);
    });

    let p2 = beams.iter().sum::<u64>();

    println!("Part 1: {p1} Part 2: {p2}");
}

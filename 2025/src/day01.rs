
pub fn run() {
    let input = include_str!("../input/day01.txt");

    let (p1, p2, _) = input.lines().fold((0, 0, 50), |(p1, p2, pos), line| {
        let (dir, val_str) = line.split_at(1);
        let val = val_str.parse::<i32>().unwrap();

        let (raw_pos, laps) = match dir {
            "L" => {
                let gap = 100 - pos;
                (pos - val, (gap + val) / 100 - (gap / 100))
            }
            _ => (pos + val, (pos + val) / 100),
        };

        let new_pos = raw_pos.rem_euclid(100);

        (p1 + i32::from(new_pos == 0), p2 + laps, new_pos)
    });

    println!("Part 1: {p1}, Part 2: {p2}");
}

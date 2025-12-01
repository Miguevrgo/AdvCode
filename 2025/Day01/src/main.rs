fn main() {
    let input = std::fs::read_to_string("input").expect("Input file not found");
    let mut dial_pos: i32 = 50;
    let (mut p1, mut p2) = (0, 0);
    input.lines().for_each(|l| {
        let initial_pos = dial_pos;
        let mut neg = false;
        if let Some(stripped) = l.strip_prefix('L') {
            dial_pos -= stripped.parse::<i32>().unwrap();
            neg = true;
        } else {
            dial_pos += l[1..].parse::<i32>().unwrap();
        }
        let distance = (initial_pos - dial_pos).abs();
        if neg {
            p2 += ((100 - initial_pos) + distance).div_euclid(100);
            p2 -= (100 - initial_pos) / 100;
        } else {
            p2 += (initial_pos + distance).div_euclid(100);
        }
        dial_pos = dial_pos.rem_euclid(100);

        if (dial_pos % 100) == 0 {
            p1 += 1;
        }
    });

    println!("Part 1: {p1}, Part 2: {p2}");
}

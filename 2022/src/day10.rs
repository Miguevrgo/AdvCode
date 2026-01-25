pub fn run() {
    let input = include_str!("../input/day10.txt");
    let mut x = 1;
    let mut history = Vec::with_capacity(240);

    for line in input.lines() {
        history.push(x);

        if let Some(val_str) = line.strip_prefix("addx ") {
            history.push(x);
            x += val_str.parse::<i32>().unwrap();
        }
    }

    let p1: i32 = history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as i32 + 1, val))
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(cycle, val)| cycle * val)
        .sum();

    println!("Part 1: {p1}");
    println!("Part 2:");

    for (i, &sprite_pos) in history.iter().enumerate().take(240) {
        let crt_x = (i as i32) % 40;

        if (crt_x - sprite_pos).abs() <= 1 {
            print!("█");
        } else {
            print!(" ");
        }

        if (i + 1) % 40 == 0 {
            println!();
        }
    }
}

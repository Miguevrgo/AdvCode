pub fn run() {
    let input = include_bytes!("../input/day06.txt");

    let pos = |window_size: usize| -> usize {
        let mut last_seen = [0; 26];
        let mut window_start = 0;
        for (i, &byte) in input.iter().enumerate() {
            if byte == b'\n' {
                continue;
            }

            let idx = (byte - b'a') as usize;

            if last_seen[idx] > window_start {
                window_start = last_seen[idx];
            }

            last_seen[idx] = i + 1;

            if i + 1 - window_start == window_size {
                return i + 1;
            }
        }
        unreachable!()
    };

    println!("Part 1: {}, Part 2: {}", pos(4), pos(14));
}

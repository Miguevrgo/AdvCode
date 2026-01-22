pub fn run() {
    let input = include_str!("../input/day07.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut sizes = Vec::new();
    let mut stack = Vec::new();
    let mut total = 0;

    for token in lines {
        if token.starts_with("$ cd") {
            if token.ends_with("..") {
                sizes.push(total);
                total += stack.pop().unwrap();
            } else {
                stack.push(total);
                total = 0;
            }
        } else if token.as_bytes()[0].is_ascii_digit() {
            total += token
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    }

    while let Some(prev) = stack.pop() {
        sizes.push(total);
        total += prev;
    }

    let free = 70_000_000 - total;

    let p1: u32 = sizes.iter().filter(|&&dir| dir <= 100000).sum();
    let p2 = sizes
        .iter()
        .filter(|&&dir| dir + free >= 30_000_000)
        .min()
        .unwrap();
    println!("Part 1: {p1} Part 2: {p2}");
}

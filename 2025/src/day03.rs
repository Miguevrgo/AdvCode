pub fn run() {
    let input = include_str!("../input/day03.txt");
    let banks: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let (p1, p2): (u64, u64) = banks.iter().fold((0, 0), |(acc1, acc2), &bank| {
        let solve = |tops: &mut [u8]| -> u64 {
            for (i, &rating) in bank.iter().enumerate() {
                let mut pos = tops.len().saturating_sub(bank.len() - i);

                while pos < tops.len() {
                    if rating > tops[pos] {
                        tops[pos] = rating;
                        tops[(pos + 1)..].fill(0);
                        break;
                    }
                    pos += 1;
                }
            }
            tops.iter()
                .fold(0, |acc, &d| acc * 10 + d.saturating_sub(b'0') as u64)
        };

        (acc1 + solve(&mut [0u8; 2]), acc2 + solve(&mut [0u8; 12]))
    });

    println!("Part 1: {p1}, Part 2: {p2}");
}

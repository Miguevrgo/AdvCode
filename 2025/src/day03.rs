use std::mem::replace;
pub fn run() {
    let input = include_str!("../input/day03.txt");
    let banks: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let (p1, p2): (u64, u64) = banks.iter().fold((0, 0), |(acc1, acc2), bank| {
        let solve = |tops: &mut [u8]| -> u64 {
            let mut final_batteries = vec![0; tops.len()];
            let wall = bank.len() - tops.len();
            final_batteries.copy_from_slice(&bank[wall..]);

            for mut rating in bank[0..wall].iter().rev().copied() {
                for battery in &mut final_batteries {
                    if rating < *battery {
                        break;
                    }

                    rating = replace(battery, rating);
                }
            }

            final_batteries
                .iter()
                .fold(0, |acc, &d| acc * 10 + d.saturating_sub(b'0') as u64)
        };

        (acc1 + solve(&mut [0u8; 2]), acc2 + solve(&mut [0u8; 12]))
    });

    println!("Part 1: {p1}, Part 2: {p2}");
}

use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../input/day02.txt");
    let ranges: Vec<(i64, i64)> = input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .map(|(l, r)| (l.trim().parse().unwrap(), r.trim().parse().unwrap()))
        .collect();

    let global_min = ranges.iter().map(|r| r.0).min().unwrap_or(0);
    let global_max = ranges.iter().map(|r| r.1).max().unwrap_or(0);
    let mut p2_found = HashSet::new();

    let min_digits = global_min.checked_ilog10().unwrap_or(0) + 1;
    let max_digits = global_max.checked_ilog10().unwrap_or(0) + 1;

    let mut p1 = 0;
    for len in min_digits..=max_digits {
        for chunk_len in 1..=(len / 2) {
            if len % chunk_len != 0 {
                continue;
            }

            let block_pow = 10_i64.pow(chunk_len);
            let multiplier = (10_i64.pow(len) - 1) / (block_pow - 1);

            let min_base = 10_i64.pow(chunk_len - 1);
            let max_base = block_pow - 1;

            let start = min_base.max((global_min + multiplier - 1) / multiplier);
            let end = max_base.min(global_max / multiplier);

            for base in start..=end {
                let val = base * multiplier;

                if ranges.iter().any(|&(l, r)| val >= l && val <= r) {
                    if len / chunk_len == 2 {
                        p1 += val;
                    }
                    p2_found.insert(val);
                }
            }
        }
    }

    let p2 = p2_found.iter().sum::<i64>();
    println!("Part 1: {p1} Part 2: {p2}");
}

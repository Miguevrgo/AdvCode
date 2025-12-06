pub fn run() {
    let input = include_str!("../input/day06.txt");
    let mut lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let op_str = lines.pop().unwrap();

    let op_indices: Vec<(usize, char)> = op_str
        .char_indices()
        .filter(|(_, ch)| !ch.is_whitespace())
        .collect();

    let mut ranges = Vec::with_capacity(op_indices.len());
    let mut operations = Vec::with_capacity(op_indices.len());

    for i in 0..op_indices.len() {
        let (start, op) = op_indices[i];
        let end = op_indices.get(i + 1).map(|p| p.0).unwrap_or(op_str.len());

        ranges.push(start..end);
        operations.push(op);
    }

    let mut cols = vec![Vec::with_capacity(lines.len()); ranges.len()];

    for l in lines {
        for (i, r) in ranges.iter().enumerate() {
            let start = r.start;
            let end = r.end.min(l.len());

            cols[i].push(&l[start..end]);
        }
    }

    let p1: u64 = cols
        .iter()
        .enumerate()
        .map(|(i, col)| {
            if operations[i] == '+' {
                col.iter()
                    .map(|n| n.trim().parse::<u64>().unwrap())
                    .sum::<u64>()
            } else {
                col.iter()
                    .map(|n| n.trim().parse::<u64>().unwrap())
                    .product::<u64>()
            }
        })
        .sum();

    let p2: u64 = cols
        .iter()
        .zip(&operations)
        .map(|(col, &op)| {
            let mut nums = [0; 4];

            for s in col {
                for (i, b) in s.bytes().enumerate().filter(|(_, b)| *b != b' ') {
                    nums[i] = nums[i] * 10 + (b - b'0') as u64;
                }
            }

            if op == '+' {
                nums.iter().sum::<u64>()
            } else {
                nums.iter().filter(|&&n| n != 0).product()
            }
        })
        .sum();

    println!("Part 1: {p1}, Part 2: {p2}");
}

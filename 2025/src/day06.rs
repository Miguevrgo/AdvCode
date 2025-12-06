const NUM_ROWS: u32 = 4;

pub fn run() {
    let input = include_str!("../input/day06.txt");
    let mut cols: Vec<Vec<String>> = Vec::new();
    let mut operations = Vec::new();
    let mut num_digits: Vec<u8> = Vec::new();

    let op_str = input.lines().last().unwrap();
    let mut spaces = 0;

    for ch in op_str.chars() {
        if ch.is_whitespace() {
            spaces += 1;
        } else {
            if !operations.is_empty() {
                num_digits.push(spaces);
            }
            operations.push(ch);
            spaces = 0;
        }
    }
    num_digits.push(spaces + 1);

    for (l_num, l) in input.lines().enumerate() {
        if l_num < NUM_ROWS as usize {
            let mut current = 0;
            for (i, &jump) in num_digits.iter().enumerate() {
                let slice = &l[current..current + jump as usize];
                current += jump as usize + 1;
                if cols.len() <= i {
                    cols.push(Vec::new());
                }
                cols[i].push(slice.to_string())
            }
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
        .enumerate()
        .map(|(i, col)| {
            if operations[i] == '+' {
                let mut nums = [0; 4];
                for elem in col.iter() {
                    for (digit, ch) in elem.chars().enumerate() {
                        if !ch.is_whitespace() {
                            nums[digit] *= 10;
                            nums[digit] += ch.to_digit(10).unwrap() as u64;
                        }
                    }
                }
                nums.iter().sum::<u64>()
            } else {
                let mut nums = [0; 4];
                for elem in col.iter() {
                    for (digit, ch) in elem.chars().enumerate() {
                        if !ch.is_whitespace() {
                            nums[digit] *= 10;
                            nums[digit] += ch.to_digit(10).unwrap() as u64;
                        }
                    }
                }
                nums.iter().filter(|&&n| n != 0).product::<u64>()
            }
        })
        .sum();

    println!("Part 1: {p1}, Part 2: {p2}");
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to find input file");

    let banks: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            let mut bank = Vec::with_capacity(l.len());
            for ch in l.chars() {
                bank.push(ch.to_digit(10).unwrap() as u8);
            }

            bank
        })
        .collect();

    let (mut p1, mut p2): (u64, u64) = (0, 0);
    for bank in banks {
        let mut tops = [0; 12];

        for (i, &rating) in bank.iter().enumerate() {
            let mut pos = tops.len().saturating_sub(bank.len() - i);
            while pos < tops.len() {
                if rating > tops[pos] {
                    tops[pos] = rating;
                    for val in &mut tops[(pos + 1)..] {
                        *val = 0;
                    }
                    break;
                }
                pos += 1;
            }
        }

        p1 += (tops[0] * 10 + tops[1]) as u64;

        for (i, &top) in tops.iter().enumerate() {
            p2 += top as u64 * 10_u64.pow((tops.len() - i - 1) as u32)
        }
    }

    println!("Part 1: {p1}, Part 2: {p2}");
}

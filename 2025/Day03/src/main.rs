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

    let (mut p1, mut p2): (u32, u32) = (0, 0);
    for bank in banks {
        let mut tops = vec![0; 12];
        let mut top_one = (0, 0);
        let mut top_two = (0, 0);
        for (i, &rating) in bank.iter().enumerate() {
            if rating > top_two.1 {
                if rating > top_one.1 && i < bank.len() - 1 {
                    top_two = (0, 0);
                    top_one = (i, rating);
                } else {
                    top_two = (i, rating);
                }
            }
        }

        println!("top_one: {top_one:?}, top_two:{top_two:?}");
        if top_one.0 < top_two.0 {
            println!("{}", (top_one.1 * 10 + top_two.1));
            p1 += (top_one.1 * 10 + top_two.1) as u32
        } else {
            println!("{}", (top_two.1 * 10 + top_one.1));
            p1 += (top_two.1 * 10 + top_one.1) as u32
        }
    }

    println!("Part 1: {p1}, Part 2: {p2}");
}

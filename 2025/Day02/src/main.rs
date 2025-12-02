fn main() {
    let input = std::fs::read_to_string("input").expect("File input not found");
    let ranges: Vec<(i64, i64)> = input
        .trim()
        .split(',')
        .map(|rg| {
            let (left, right) = rg.split_once('-').unwrap();
            (
                left.parse::<i64>()
                    .unwrap_or_else(|_| panic!("Invalid num: {left}",)),
                right
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Invalid num: {right} |")),
            )
        })
        .collect();

    let (mut p1, mut p2) = (0, 0);
    for (first_id, last_id) in ranges {
        (first_id..=last_id).for_each(|id| {
            let str = id.to_string();
            let bytes = str.as_bytes();
            let len = bytes.len();
            if len % 2 == 0 && bytes[..len / 2] == bytes[len / 2..] {
                p1 += id;
                p2 += id;
                return;
            }

            for chunk in (1..=(len / 2)).rev() {
                if len % chunk == 0 {
                    let first = bytes.chunks(chunk).next().unwrap();
                    if !bytes.chunks(chunk).any(|ch| ch != first) {
                        p2 += id;
                        break;
                    }
                }
            }
        });
    }

    println!("Part 1: {p1} Part 2: {p2}");
}

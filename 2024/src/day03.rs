use regex::Regex;

pub fn run() {
    let input = include_str!("../input/day03.txt");
    let r = Regex::new(r"(mul\(\d+,\d+\)|do(n't)?\(\))").unwrap();
    let (mut p1, mut p2, mut ok) = (0, 0, true);

    for x in r.find_iter(input) {
        match x.as_str() {
            "do()" => ok = true,
            "don't()" => ok = false,
            x => {
                let (mul1, mul2) = x[4..x.len() - 1].split_once(',').unwrap();
                let temp = mul1.parse::<u32>().unwrap() * mul2.parse::<u32>().unwrap();
                p1 += temp;
                if ok {
                    p2 += temp;
                }
            }
        }
    }

    println!("Part 1: {p1}\nPart 2: {p2}");
}

const MODULO_MASK: i64 = (1 << 24) - 1;
const POSSIBLE_COMBINATIONS: usize = 19_usize.pow(4);

pub fn run() {
    let input = include_str!("../input/day22.txt");

    let mut p1 = 0;
    let mut p2 = vec![0; POSSIBLE_COMBINATIONS];
    let mut seen = vec![usize::MAX; POSSIBLE_COMBINATIONS];

    for (monkey_id, line) in input.lines().enumerate() {
        let mut num: i64 = line.parse().unwrap();

        let mut last_digit = vec![0; 2001];

        last_digit[0] = num % 10;

        for digit in last_digit.iter_mut().take(2000 + 1).skip(1) {
            num = (num ^ (num * 64)) & MODULO_MASK;
            num = (num ^ (num / 32)) & MODULO_MASK;
            num = (num ^ (num * 2048)) & MODULO_MASK;
            *digit = num % 10;
        }

        p1 += num;

        for i in 0..1997 {
            let d1 = (last_digit[i + 1] - last_digit[i]) + 9;
            let d2 = (last_digit[i + 2] - last_digit[i + 1]) + 9;
            let d3 = (last_digit[i + 3] - last_digit[i + 2]) + 9;
            let d4 = (last_digit[i + 4] - last_digit[i + 3]) + 9;

            let idx = (d1 * 6859 + d2 * 361 + d3 * 19 + d4) as usize;

            if seen[idx] != monkey_id {
                p2[idx] += last_digit[i + 4];
                seen[idx] = monkey_id;
            }
        }
    }

    println!("Part 1: {p1}, Part 2: {}", p2.into_iter().max().unwrap());
}

use std::collections::HashMap;

const fn str_to_u32(s: &str) -> u32 {
    let bytes = s.as_bytes();
    (bytes[0] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[2] as u32)
}

const OUT_KEY: u32 = str_to_u32("out");
const DAC_KEY: u32 = str_to_u32("dac");
const FFT_KEY: u32 = str_to_u32("fft");
const YOU_KEY: u32 = str_to_u32("you");
const SVR_KEY: u32 = str_to_u32("svr");

fn count(
    cache: &mut HashMap<(u32, bool, bool), u64>,
    node_key: u32,
    server: &HashMap<u32, Vec<u32>>,
    mut dac_visited: bool,
    mut fft_visited: bool,
) -> u64 {
    if node_key == OUT_KEY {
        return u64::from(dac_visited && fft_visited);
    }

    dac_visited |= node_key == DAC_KEY;
    fft_visited |= node_key == FFT_KEY;

    let key = (node_key, dac_visited, fft_visited);

    if !cache.contains_key(&key) {
        let result = server[&node_key]
            .iter()
            .map(|word| count(cache, *word, server, dac_visited, fft_visited))
            .sum();

        cache.insert(key, result);
    }

    cache[&key]
}

pub fn run() {
    let input = include_str!("../input/day11.txt");

    let graph: HashMap<u32, Vec<u32>> = input
        .lines()
        .map(|line| {
            let (left_str, right_str) = line.split_once(':').unwrap();
            let key = str_to_u32(left_str);
            let destinations = right_str
                .split_ascii_whitespace()
                .map(str_to_u32)
                .collect::<Vec<u32>>();
            (key, destinations)
        })
        .collect();

    let mut cache = HashMap::new();
    let p1 = count(&mut cache, YOU_KEY, &graph, true, true);
    let p2 = count(&mut cache, SVR_KEY, &graph, false, false);

    println!("Part 1: {p1}, Part 2: {p2}");
}


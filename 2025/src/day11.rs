use std::collections::HashMap;

const fn to_u16(b: &[u8]) -> u16 {
    ((b[0] - b'a') as u16) << 10 | ((b[1] - b'a') as u16) << 5 | ((b[2] - b'a') as u16)
}

const OUT_KEY: u16 = to_u16(b"out");
const DAC_KEY: u16 = to_u16(b"dac");
const FFT_KEY: u16 = to_u16(b"fft");
const YOU_KEY: u16 = to_u16(b"you");
const SVR_KEY: u16 = to_u16(b"svr");

const DAC_MASK: u16 = 0x8000;
const ID_MASK: u16 = 0x7FFF;

fn count(
    cache: &mut HashMap<u32, u64>,
    current: u16,
    fft_visited: bool,
    graph: &HashMap<u16, Vec<u16>>,
) -> u64 {
    let node_id = current & ID_MASK;
    let dac_visited = (current & DAC_MASK) != 0;

    if node_id == OUT_KEY {
        return u64::from(dac_visited && fft_visited);
    }

    let next_fft_visited = fft_visited | (node_id == FFT_KEY);
    let key = (current as u32) | ((next_fft_visited as u32) << 16);

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = graph.get(&node_id).map_or(0, |neighbors| {
        neighbors
            .iter()
            .map(|&neighbor| {
                let next_dac_mask = if dac_visited || neighbor == DAC_KEY {
                    DAC_MASK
                } else {
                    0
                };
                count(cache, neighbor | next_dac_mask, next_fft_visited, graph)
            })
            .sum()
    });

    cache.insert(key, result);
    result
}

pub fn run() {
    let graph: HashMap<_, _> = include_bytes!("../input/day11.txt")
        .split(|&b| b == b'\n')
        .filter(|line| line.len() > 4)
        .map(|line| {
            (
                to_u16(&line[..3]),
                line[5..].split(|&b| b == b' ').map(to_u16).collect(),
            )
        })
        .collect();

    let mut cache = HashMap::with_capacity(2048);
    let p1 = count(&mut cache, YOU_KEY | DAC_MASK, true, &graph);
    let p2 = count(&mut cache, SVR_KEY, false, &graph);

    println!("Part 1: {p1}, Part 2: {p2}");
}

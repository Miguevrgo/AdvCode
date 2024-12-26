use std::fs;

const MASK: u64 = 0b00000_11111_11111_11111_11111_11111_00000; // Máscara para 7x5

fn main() {
    let input = fs::read_to_string("input").expect("File not found");
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in input.split("\n\n") {
        let bits = block.lines().fold(0u64, |acc, line| {
            let line_bits = line
                .bytes()
                .fold(0u64, |line_acc, b| (line_acc << 1) | (b & 1) as u64);
            (acc << 5) | line_bits
        });

        if bits & 1 == 0 {
            locks.push(bits & MASK);
        } else {
            keys.push(bits & MASK);
        }
    }

    let mut p1 = 0;
    for lock in &locks {
        for key in &keys {
            p1 += (lock & key == 0) as u32;
        }
    }

    println!("Part 1: {p1}");
}

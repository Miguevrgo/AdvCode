use std::fs;

fn part1() {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let contents = fs::read_to_string("../input").unwrap();
    let strings: Vec<_> = contents.split_whitespace().collect();
    let elems: Vec<u32> = strings.iter().flat_map(|x| x.parse()).collect();

    for i in (0..elems.len()).step_by(2) {
        left.push(elems[i]);
        if i + 1 < elems.len() {
            right.push(elems[i + 1]);
        }
    }

    left.sort();
    right.sort();

    let total_distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l as i32 - r as i32).abs() as u32)
        .sum();

    println!("Total distance: {}", total_distance);
}

fn part2() {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let contents = fs::read_to_string("../input").unwrap();
    let strings: Vec<_> = contents.split_whitespace().collect();
    let elems: Vec<u32> = strings.iter().flat_map(|x| x.parse()).collect();
    let mut total_distance = 0;

    for i in (0..elems.len()).step_by(2) {
        left.push(elems[i]);
        if i + 1 < elems.len() {
            right.push(elems[i + 1]);
        }
    }

    left.sort();
    right.sort();

    let mut j = 0;

    for elem in left {
        let mut counter = 0;

        while j < right.len() && right[j] <= elem {
            if right[j] == elem {
                counter += 1;
            }
            j += 1;
        }

        total_distance += counter * elem;
    }

    println!("Final total distance: {}", total_distance);
}

fn main() {
    part1();
    part2();
}

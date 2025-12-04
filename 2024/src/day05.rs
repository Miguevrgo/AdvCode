use std::collections::HashMap;

fn part1() {
    let mut sum: u32 = 0;
    let input = include_str!("../input/day05.txt");
    let contents: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in contents[0].lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x = x.parse::<u32>().expect("Error parsing X");
        let y = y.parse::<u32>().expect("Error parsing Y");
        rules.entry(x).or_default().push(y);
    }

    let pages: Vec<Vec<u32>> = contents[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().expect("Error parsing"))
                .collect()
        })
        .collect();

    'outer: for line in &pages {
        let mut positions: HashMap<u32, usize> = HashMap::new();
        for (index, &page) in line.iter().enumerate() {
            positions.insert(page, index);
        }

        for (&x, ys) in &rules {
            if let Some(&x_pos) = positions.get(&x) {
                for &y in ys {
                    if let Some(&y_pos) = positions.get(&y) {
                        if x_pos > y_pos {
                            continue 'outer;
                        }
                    }
                }
            }
        }

        let middle_index = line.len() / 2;
        sum += line[middle_index];
    }

    println!("{}", sum);
}

fn correct_order(mut line: Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut changed = true;

    while changed {
        changed = false;

        for (&x, ys) in rules {
            if let Some(x_pos) = line.iter().position(|&p| p == x) {
                for &y in ys {
                    if let Some(y_pos) = line.iter().position(|&p| p == y) {
                        if x_pos > y_pos {
                            line.swap(x_pos, y_pos);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    line
}

fn part2() {
    let mut sum: u32 = 0;
    let input = include_str!("../input/day05.txt");
    let contents: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in contents[0].lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x = x.parse::<u32>().expect("Error parsing X");
        let y = y.parse::<u32>().expect("Error parsing Y");
        rules.entry(x).or_default().push(y);
    }

    let pages: Vec<Vec<u32>> = contents[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().expect("Error parsing"))
                .collect()
        })
        .collect();

    for line in &pages {
        let mut positions: HashMap<u32, usize> = HashMap::new();
        for (index, &page) in line.iter().enumerate() {
            positions.insert(page, index);
        }

        let mut is_valid = true;
        'outer: for (&x, ys) in &rules {
            if let Some(&x_pos) = positions.get(&x) {
                for &y in ys {
                    if let Some(&y_pos) = positions.get(&y) {
                        if x_pos > y_pos {
                            is_valid = false;
                            break 'outer;
                        }
                    }
                }
            }
        }

        if !is_valid {
            let ordered = correct_order(line.clone(), &rules);
            let middle_index = ordered.len() / 2;
            sum += ordered[middle_index];
        }
    }

    println!("{}", sum);
}

pub fn run() {
    part1();
    part2();
}

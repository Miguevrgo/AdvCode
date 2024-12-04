use std::fs;

// Don't look, it's embarrasing :(
fn part1() {
    let mut count = 0;
    let contents = fs::read_to_string("./input").unwrap();
    let strings: Vec<Vec<char>> = contents
        .lines()
        .map(|lines| lines.chars().collect())
        .collect();

    for i in 0..strings.len() {
        for j in 0..strings[i].len() {
            if strings[i][j] == 'X' {
                if j + 3 < strings[i].len() // Right
                    && strings[i][j + 1] == 'M'
                    && strings[i][j + 2] == 'A'
                    && strings[i][j + 3] == 'S'
                {
                    count += 1;
                }
                if i as isize - 3 >= 0 // Top Right
                    && j + 3 < strings[i].len()
                    && strings[i - 1][j + 1] == 'M'
                    && strings[i - 2][j + 2] == 'A'
                    && strings[i - 3][j + 3] == 'S'
                {
                    count += 1;
                }
                if j as isize - 3 >= 0 // Left
                    && strings[i][j - 1] == 'M'
                    && strings[i][j - 2] == 'A'
                    && strings[i][j - 3] == 'S'
                {
                    count += 1;
                }
                if i as isize - 3 >= 0 // Top left
                    && j as isize - 3 >= 0
                    && strings[i - 1][j - 1] == 'M'
                    && strings[i - 2][j - 2] == 'A'
                    && strings[i - 3][j - 3] == 'S'
                {
                    count += 1;
                }
                if i + 3 < strings.len() // Bottom
                    && strings[i + 1][j] == 'M'
                    && strings[i + 2][j] == 'A'
                    && strings[i + 3][j] == 'S'
                {
                    count += 1;
                }
                if i as isize - 3 >= 0 // Top
                    && strings[i - 1][j] == 'M'
                    && strings[i - 2][j] == 'A'
                    && strings[i - 3][j] == 'S'
                {
                    count += 1;
                }
                if i + 3 < strings.len() // Bottom Right
                    && j + 3 < strings[i].len()
                    && strings[i + 1][j + 1] == 'M'
                    && strings[i + 2][j + 2] == 'A'
                    && strings[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }
                if i + 3 < strings.len() // Bottom Left
                    && j as isize - 3 >= 0
                    && strings[i + 1][j - 1] == 'M'
                    && strings[i + 2][j - 2] == 'A'
                    && strings[i + 3][j - 3] == 'S'
                {
                    count += 1;
                }
            };
        }
    }

    println!("Part 1: {count}")
}

fn part2() {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let contents = fs::read_to_string("./input").unwrap();
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

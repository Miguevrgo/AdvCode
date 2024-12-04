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
    let mut count = 0;

    let contents = fs::read_to_string("./input").unwrap();
    let strings: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mas_patterns = ["MAS", "SAM"];

    for i in 1..strings.len() - 1 {
        for j in 1..strings[i].len() - 1 {
            let left = [strings[i - 1][j - 1], strings[i][j], strings[i + 1][j + 1]]
                .iter()
                .collect::<String>();

            let right = [strings[i - 1][j + 1], strings[i][j], strings[i + 1][j - 1]]
                .iter()
                .collect::<String>();

            if mas_patterns.contains(&left.as_str()) && mas_patterns.contains(&right.as_str()) {
                count += 1;
            }
        }
    }

    println!("Part 2: {count}");
}

fn main() {
    part1();
    part2();
}

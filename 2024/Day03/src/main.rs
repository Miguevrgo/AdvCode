use regex::Regex;
use std::fs;

fn part1() {
    let mut sum: i32 = 0;
    let contents = fs::read_to_string("./input").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [num1, num2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}

fn part2() {
    let mut sum: i32 = 0;
    let mut mul_enabled = true;
    let contents = fs::read_to_string("./input").unwrap();
    let re = Regex::new(r"(do\(\)|don't\(\))|mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(&contents) {
        if let Some(instruction) = cap.get(1) {
            if instruction.as_str() == "do()" {
                mul_enabled = true;
            } else if instruction.as_str() == "don't()" {
                mul_enabled = false;
            }
        } else if mul_enabled {
            sum += cap[2].parse::<i32>().unwrap() * cap[3].parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}

use std::fs;

fn valid(nums: &[i64], target: i64, part2: bool) -> i64 {
    let mut stack = vec![(0, 0)];

    while let Some((i, current)) = stack.pop() {
        if i == nums.len() && current == target {
            return target;
        }

        if i < nums.len() {
            let num = nums[i];
            stack.push((i + 1, current + num));
            stack.push((i + 1, current * num));
            if part2 {
                stack.push((i + 1, current * 10i64.pow(num.ilog10() + 1) + num));
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let (left, right): (Vec<i64>, Vec<Vec<i64>>) = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(':').unwrap();
            (
                left.trim().parse::<i64>().expect("Unable to read number"),
                right
                    .split_ascii_whitespace()
                    .map(|num| num.trim().parse::<i64>().expect("Unable to read number"))
                    .collect(),
            )
        })
        .unzip();

    let p1: i64 = left
        .iter()
        .zip(&right)
        .map(|(&test_val, nums)| valid(nums, test_val, false))
        .sum();
    let p2: i64 = left
        .iter()
        .zip(&right)
        .map(|(&test_val, nums)| valid(nums, test_val, true))
        .sum();

    println!("Part 1: {p1}\nPart 2: {p2}");
}

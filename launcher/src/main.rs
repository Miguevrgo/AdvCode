use std::env;
use std::time::Instant;

use gag::Gag;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filter = if args.len() > 1 { args[1].as_str() } else { "" };

    let total_start = Instant::now();
    println!(" Advent of Code  ");

    if filter.is_empty() || filter == "2022" {
        println!("\n  2022");
        measure("Day 01", year2022::day01::run);
        measure("Day 02", year2022::day02::run);
        measure("Day 03", year2022::day03::run);
        measure("Day 04", year2022::day04::run);
        measure("Day 05", year2022::day05::run);
        measure("Day 06", year2022::day06::run);
        measure("Day 07", year2022::day07::run);
        measure("Day 08", year2022::day08::run);
        measure("Day 09", year2022::day09::run);
        measure("Day 10", year2022::day10::run);
    }

    if filter.is_empty() || filter == "2024" {
        println!("\n  2024");
        measure("Day 01", year2024::day01::run);
        measure("Day 02", year2024::day02::run);
        measure("Day 03", year2024::day03::run);
        measure("Day 04", year2024::day04::run);
        measure("Day 05", year2024::day05::run);
        measure("Day 06", year2024::day06::run);
        measure("Day 07", year2024::day07::run);
        measure("Day 08", year2024::day08::run);
        measure("Day 09", year2024::day09::run);
        measure("Day 10", year2024::day10::run);
        measure("Day 11", year2024::day11::run);
        measure("Day 12", year2024::day12::run);
        measure("Day 13", year2024::day13::run);
        measure("Day 14", year2024::day14::run);
        measure("Day 18", year2024::day18::run);
        measure("Day 20", year2024::day20::run);
        measure("Day 22", year2024::day22::run);
        measure("Day 25", year2024::day25::run);
    }

    if filter.is_empty() || filter == "2025" {
        println!("\n  2025");
        measure("Day 01", year2025::day01::run);
        measure("Day 02", year2025::day02::run);
        measure("Day 03", year2025::day03::run);
        measure("Day 04", year2025::day04::run);
        measure("Day 05", year2025::day05::run);
        measure("Day 06", year2025::day06::run);
        measure("Day 07", year2025::day07::run);
        measure("Day 08", year2025::day08::run);
        measure("Day 09", year2025::day09::run);
        measure("Day 10", year2025::day10::run);
        measure("Day 11", year2025::day11::run);
        measure("Day 12", year2025::day12::run);
    }

    println!("\n󱐋 Total Time: {:.2?}", total_start.elapsed());
}

fn measure<F: FnOnce()>(name: &str, f: F) {
    let start = Instant::now();
    {
        let _print_gag = Gag::stdout().ok();
        f();
    }
    println!("   - {}: \t{:.2?}", name, start.elapsed());
}

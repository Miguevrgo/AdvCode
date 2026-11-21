use std::env;
use std::time::Instant;

use gag::Gag;

mod year2021;
mod year2023;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args.get(1).map(String::as_str);
    let day: Option<u32> = args.get(2).and_then(|day| day.parse().ok());
    let wants = |name: &str| year.is_none_or(|year| year == name);
    let measure = |number: u32, run: fn()| {
        if day.is_none_or(|day| day == number) {
            timed(number, run);
        }
    };

    let total_start = Instant::now();
    println!(" Advent of Code  ");

    if wants("2021") {
        println!("\n  2021");
        if let Err(error) = year2021::run(day) {
            eprintln!("2021: {error}");
            std::process::exit(1);
        }
    }

    if wants("2022") {
        println!("\n  2022");
        measure(1, year2022::day01::run);
        measure(2, year2022::day02::run);
        measure(3, year2022::day03::run);
        measure(4, year2022::day04::run);
        measure(5, year2022::day05::run);
        measure(6, year2022::day06::run);
        measure(7, year2022::day07::run);
        measure(8, year2022::day08::run);
        measure(9, year2022::day09::run);
        measure(10, year2022::day10::run);
        measure(12, year2022::day12::run);
    }

    if wants("2023") {
        println!("\n  2023");
        if let Err(error) = year2023::run(day) {
            eprintln!("2023: {error}");
            std::process::exit(1);
        }
    }

    if wants("2024") {
        println!("\n  2024");
        measure(1, year2024::day01::run);
        measure(2, year2024::day02::run);
        measure(3, year2024::day03::run);
        measure(4, year2024::day04::run);
        measure(5, year2024::day05::run);
        measure(6, year2024::day06::run);
        measure(7, year2024::day07::run);
        measure(8, year2024::day08::run);
        measure(9, year2024::day09::run);
        measure(10, year2024::day10::run);
        measure(11, year2024::day11::run);
        measure(12, year2024::day12::run);
        measure(13, year2024::day13::run);
        measure(14, year2024::day14::run);
        measure(18, year2024::day18::run);
        measure(20, year2024::day20::run);
        measure(22, year2024::day22::run);
        measure(25, year2024::day25::run);
    }

    if wants("2025") {
        println!("\n  2025");
        measure(1, year2025::day01::run);
        measure(2, year2025::day02::run);
        measure(3, year2025::day03::run);
        measure(4, year2025::day04::run);
        measure(5, year2025::day05::run);
        measure(6, year2025::day06::run);
        measure(7, year2025::day07::run);
        measure(8, year2025::day08::run);
        measure(9, year2025::day09::run);
        measure(10, year2025::day10::run);
        measure(11, year2025::day11::run);
        measure(12, year2025::day12::run);
    }

    println!("\n󱐋 Total Time: {:.2?}", total_start.elapsed());
}

fn timed(day: u32, run: fn()) {
    let start = Instant::now();
    {
        let _print_gag = Gag::stdout().ok();
        run();
    }
    println!("   - Day {day:02}: \t{:.2?}", start.elapsed());
}

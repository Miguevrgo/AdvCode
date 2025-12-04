use std::env;
use std::time::Instant;
use year2022::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <day_number>");
        println!("Example: cargo run -- 01");
        return;
    }

    let day = args[1].as_str();
    let start = Instant::now();

    match day {
        "01" | "1" => crate::day01::run(),
        "02" | "2" => crate::day02::run(),
        "03" | "3" => crate::day03::run(),
        "04" | "4" => crate::day04::run(),
        "05" | "5" => crate::day05::run(),
        _ => println!("Day not implemented or not found"),
    }

    println!("Time elapsed: {:.2?}", start.elapsed());
}

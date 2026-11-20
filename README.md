# Advent of Code

Personal solutions to the yearly [Advent of Code](https://adventofcode.com/). A challenge I started doing in 2023. I´ve
ended up loving the challenges and the learning it provides, encouraging me to complete it every year trying to improve
my skills in the journey.

- In 2023 I tried to make it in Modern C++
- In 2024 I tried to make it in Rust
- In 2025 I tried to make it in Rust
- I have also tried some other past years, mostly in Rust

I hope it results useful to anyone wanting to compare solutions and learn new ways of approaching the same problem.

## Benchmarks

Run `cargo run --release` from the repository root to benchmark all years, or
`cargo run --release -- 2023` to benchmark 2023. The 2023 benchmarks use `c++`
(override with `CXX`) to compile with `-std=c++17 -O3 -march=native -flto`.

## 2025

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Secret Entrance](https://adventofcode.com/2025/day/1) | [Source](2025/src/day01.rs) | 42 |
| 2 | [Gift Shop](https://adventofcode.com/2025/day/2) | [Source](2025/src/day02.rs) | 1080 |
| 3 | [Lobby](https://adventofcode.com/2025/day/3) | [Source](2025/src/day03.rs) | 55 |
| 4 | [Printing Department](https://adventofcode.com/2025/day/4) | [Source](2025/src/day04.rs) | 191 |
| 5 | [Cafeteria](https://adventofcode.com/2025/day/5) | [Source](2025/src/day05.rs) | 98 |
| 6 | [Trash Compactor](https://adventofcode.com/2025/day/6) | [Source](2025/src/day06.rs) | 76 |
| 7 | [Laboratories](https://adventofcode.com/2025/day/7) | [Source](2025/src/day07.rs) | 35 |
| 8 | [Playground](https://adventofcode.com/2025/day/8) | [Source](2025/src/day08.rs) | 1620 |
| 9 | [Movie Theater](https://adventofcode.com/2025/day/9) | [Source](2025/src/day09.rs) | 327 |
| 10 | [Factory](https://adventofcode.com/2025/day/10) | [Source](2025/src/day10.rs) | 16640 |
| 11 | [Reactor](https://adventofcode.com/2025/day/11) | [Source](2025/src/day11.rs) | 100 |
| 12 | [Christmas Tree Farm](https://adventofcode.com/2025/day/12) | [Source](2025/src/day12.rs) | 53 |

## 2024

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Source](2024/src/day01.rs) | 76 |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2) | [Source](2024/src/day02.rs) | 109 |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3) | [Source](2024/src/day03.rs) | 257 |
| 4 | [Ceres Search](https://adventofcode.com/2024/day/4) | [Source](2024/src/day04.rs) | 530 |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5) | [Source](2024/src/day05.rs) | 2050 |
| 6 | [Guard Gallivant](https://adventofcode.com/2024/day/6) | [Source](2024/src/day06.rs) | 38110 |
| 7 | [Bridge Repair](https://adventofcode.com/2024/day/7) | [Source](2024/src/day07.rs) | 37400 |
| 8 | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [Source](2024/src/day08.rs) | 79 |
| 9 | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | [Source](2024/src/day09.rs) | 20250 |
| 10 | [Hoof It](https://adventofcode.com/2024/day/10) | [Source](2024/src/day10.rs) | 122 |
| 11 | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | [Source](2024/src/day11.rs) | 3120 |
| 12 | [Garden Groups](https://adventofcode.com/2024/day/12) | [Source](2024/src/day12.rs) | 6210 |
| 13 | [Claw Contraption](https://adventofcode.com/2024/day/13) | [Source](2024/src/day13.rs) | 37 |
| 14 | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | [Source](2024/src/day14.rs) | 6330 |
| 18 | [RAM Run](https://adventofcode.com/2024/day/18) | [Source](2024/src/day18.rs) | 248 |
| 20 | [Race Condition](https://adventofcode.com/2024/day/20) | [Source](2024/src/day20.rs) | 16080 |
| 22 | [Monkey Market](https://adventofcode.com/2024/day/22) | [Source](2024/src/day22.rs) | 12310 |
| 25 | [Code Chronicle](https://adventofcode.com/2024/day/25) | [Source](2024/src/day25.rs) | 25 |

## 2023

Times are the median of five runs on an AMD Ryzen 7 9700X with GCC 16.2.1. Each
day's time includes reading its input, and Day 1 adds the times for its two
parts. Compilation and process startup count only toward the launcher's total.
Day 2 Part 2 and Day 14 are still incomplete.

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Trebuchet?!](https://adventofcode.com/2023/day/1) | [Source](2023/Day01/problem1.cpp) | 364 |
| 2 | [Cube Conundrum](https://adventofcode.com/2023/day/2) | [Source](2023/Day02/problem2.cpp) | 80 |
| 3 | [Gear Ratios](https://adventofcode.com/2023/day/3) | [Source](2023/Day03/problem3.cpp) | 102 |
| 4 | [Scratchcards](https://adventofcode.com/2023/day/4) | [Source](2023/Day04/problem4.cpp) | 1040 |
| 5 | [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5) | [Source](2023/Day05/problem5.cpp) | 141 |
| 6 | [Wait For It](https://adventofcode.com/2023/day/6) | [Source](2023/Day06/problem6.cpp) | 3470 |
| 7 | [Camel Cards](https://adventofcode.com/2023/day/7) | [Source](2023/Day07/problem7.cpp) | 757 |
| 8 | [Haunted Wasteland](https://adventofcode.com/2023/day/8) | [Source](2023/Day08/problem8.cpp) | 2440 |
| 9 | [Mirage Maintenance](https://adventofcode.com/2023/day/9) | [Source](2023/Day09/problem9.cpp) | 374 |
| 10 | [Pipe Maze](https://adventofcode.com/2023/day/10) | [Source](2023/Day10/problem10.cpp) | 3610 |
| 11 | [Cosmic Expansion](https://adventofcode.com/2023/day/11) | [Source](2023/Day11/problem11.cpp) | 771 |
| 12 | [Hot Springs](https://adventofcode.com/2023/day/12) | [Source](2023/Day12/problem12.cpp) | 340370 |
| 13 | [Point of Incidence](https://adventofcode.com/2023/day/13) | [Source](2023/Day13/problem13.cpp) | 390 |
| 14 | [Parabolic Reflector Dish](https://adventofcode.com/2023/day/14) | [Source](2023/Day14/problem14.cpp) | 27 |

## 2022

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Calorie Counting](https://adventofcode.com/2022/day/1) | [Source](2022/src/day01.rs) | 35 |
| 2 | [Rock Paper Scissors](https://adventofcode.com/2022/day/2) | [Source](2022/src/day02.rs) | 47 |
| 3 | [Rucksack Reorganization](https://adventofcode.com/2022/day/3) | [Source](2022/src/day03.rs) | 39 |
| 4 | [Camp Cleanup](https://adventofcode.com/2022/day/4) | [Source](2022/src/day04.rs) | 24 |
| 5 | [Supply Stacks](https://adventofcode.com/2022/day/5) | [Source](2022/src/day05.rs) | 36 |
| 6 | [Tuning Trouble](https://adventofcode.com/2022/day/6) | [Source](2022/src/day06.rs) | 6 |
| 7 | [No Space Left On Device](https://adventofcode.com/2022/day/7) | [Source](2022/src/day07.rs) | 32 |
| 8 | [Treetop Tree House](https://adventofcode.com/2022/day/8) | [Source](2022/src/day08.rs) | 315 |
| 9 | [Rope Bridge](https://adventofcode.com/2022/day/9) | [Source](2022/src/day09.rs) | 490 |
| 10 | [Cathode-Ray Tube](https://adventofcode.com/2022/day/10) | [Source](2022/src/day10.rs) | 11 |
| 12 | [Hill Climbing Algorithm](https://adventofcode.com/2022/day/12) | [Source](2022/src/day12.rs) | 114 |

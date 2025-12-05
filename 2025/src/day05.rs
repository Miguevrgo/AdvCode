pub fn run() {
    let input = include_str!("../input/day05.txt");
    let (fresh_ranges, ingredients) = input.split_once("\n\n").expect("Unable to split");

    let mut ranges = Vec::with_capacity(128);

    for (lim_inf, lim_sup) in fresh_ranges.lines().map(|l| l.split_once('-').unwrap()) {
        let left = lim_inf.parse::<u64>().unwrap();
        let right = lim_sup.parse::<u64>().unwrap();
        ranges.push(left..right + 1);
    }

    ranges.sort_unstable_by(|r1, r2| r1.start.cmp(&r2.start));

    let p1 = ingredients
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    let mut current_max = 0;
    let p2 = ranges.iter().fold(0, |acc, range| {
        let start = current_max.max(range.start);
        if start < range.end {
            let count = range.end - start;
            current_max = range.end;
            acc + count
        } else {
            acc
        }
    });

    println!("Part 1: {p1}, Part 2: {p2}")
}

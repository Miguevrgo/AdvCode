pub fn run() {
    let input = include_str!("../input/day09.txt");

    let points: Vec<(i32, i32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(',').unwrap();
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .collect();

    let mut p1 = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = (points[i].0.abs_diff(points[j].0) as u64 + 1)
                * (points[i].1.abs_diff(points[j].1) as u64 + 1);
            if area > p1 {
                p1 = area;
            }
        }
    }

    println!("Part 1: {p1}");
}

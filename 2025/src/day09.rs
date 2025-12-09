// P1 explanation: The points which create the maximum area rectangle
// are always from the points which create the Convex Hull. So calculating
// The Convex Hull (which will also help P2) using Monotone chain for simplicity
// (although I recommend reading about Chans Algorithm) allows us to reduce the
// amount of squares computed
// https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain#

fn get_convex_hull(points: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut hull = Vec::with_capacity(points.len());

    let cross_p = |a: (i32, i32), b: (i32, i32), o: (i32, i32)| -> i64 {
        (a.0 - o.0) as i64 * (b.1 - o.1) as i64 - (a.1 - o.1) as i64 * (b.0 - o.0) as i64
    };

    // Lower Hull
    for &p in points {
        while hull.len() >= 2 {
            if cross_p(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
                hull.pop();
            } else {
                break;
            }
        }

        hull.push(p);
    }

    // Upper Hull
    let lower_len = hull.len();
    for &p in points.iter().rev() {
        while hull.len() >= lower_len {
            if cross_p(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
                hull.pop();
            } else {
                break;
            }
        }

        hull.push(p);
    }

    hull.pop();
    hull
}

pub fn run() {
    let input = include_str!("../input/day09.txt");

    let mut points: Vec<(i32, i32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(',').unwrap();
            (right.parse::<i32>().unwrap(), left.parse::<i32>().unwrap())
        })
        .collect();

    points.sort_unstable();

    let hull = get_convex_hull(&points);

    let p1 = hull
        .iter()
        .enumerate()
        .flat_map(|(i, &(c_x, c_y))| {
            hull[(i + 1)..].iter().map(move |&(h_x, h_y)| {
                (c_x.abs_diff(h_x) as u64 + 1) * (c_y.abs_diff(h_y) as u64 + 1)
            })
        })
        .max()
        .unwrap();

    println!("Part 1: {p1}");
}

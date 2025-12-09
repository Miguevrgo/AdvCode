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

fn solve_p2(points: &[(i32, i32)]) -> u64 {
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = points.iter().map(|&(x, y)| (x, y)).unzip();
    xs.extend([i32::MIN, i32::MAX]);
    ys.extend([i32::MIN, i32::MAX]);
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let mapped_pts: Vec<_> = points
        .iter()
        .map(|&(x, y)| {
            (
                x,
                y,
                xs.binary_search(&x).unwrap(),
                ys.binary_search(&y).unwrap(),
            )
        })
        .collect();

    let (w, h) = (xs.len(), ys.len());
    let mut grid = vec![0u8; w * h];

    for i in 0..mapped_pts.len() {
        let (_, _, xi1, yi1) = mapped_pts[i];
        let (_, _, xi2, yi2) = mapped_pts[(i + 1) % mapped_pts.len()];

        if xi1 == xi2 {
            for yi in yi1.min(yi2)..=yi1.max(yi2) {
                grid[yi * w + xi1] = 1;
            }
        } else {
            for xi in xi1.min(xi2)..=xi1.max(xi2) {
                grid[yi1 * w + xi] = 1;
            }
        }
    }

    let mut stack = vec![(0usize, 0usize)];
    if grid[0] == 0 {
        grid[0] = 2;
    }

    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                let nr = nr as usize;
                let nc = nc as usize;

                if grid[nr * w + nc] == 0 {
                    grid[nr * w + nc] = 2;
                    stack.push((nr, nc));
                }
            }
        }
    }

    let mut p_sum = vec![vec![0i32; w + 1]; h + 1];
    for r in 0..h {
        for c in 0..w {
            let val = if grid[r * w + c] != 2 { 1 } else { 0 };
            p_sum[r + 1][c + 1] = p_sum[r][c + 1] + p_sum[r + 1][c] - p_sum[r][c] + val;
        }
    }

    let mut max_area = 0;

    for (i, &(x1, y1, xi1, yi1)) in mapped_pts.iter().enumerate() {
        for &(x2, y2, xi2, yi2) in &mapped_pts[i + 1..] {
            let area_geo = (x1.abs_diff(x2) as u64 + 1) * (y1.abs_diff(y2) as u64 + 1);
            if area_geo <= max_area {
                continue;
            }

            let (rmin, rmax) = (yi1.min(yi2), yi1.max(yi2));
            let (cmin, cmax) = (xi1.min(xi2), xi1.max(xi2));

            let expected = ((rmax - rmin + 1) * (cmax - cmin + 1)) as i32;
            let actual = p_sum[rmax + 1][cmax + 1] - p_sum[rmin][cmax + 1] - p_sum[rmax + 1][cmin]
                + p_sum[rmin][cmin];

            if expected == actual {
                max_area = area_geo;
            }
        }
    }
    max_area
}

pub fn run() {
    let input = include_str!("../input/day09.txt");

    let mut points: Vec<(i32, i32)> = input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(',').unwrap();
            (left.trim().parse().unwrap(), right.trim().parse().unwrap())
        })
        .collect();

    let p2 = solve_p2(&points);

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

    println!("Part 1: {p1}, Part 2: {p2}");
}

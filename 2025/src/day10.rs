use ndarray::prelude::{Array1, Array2};

struct Machine {
    target: u16, // Each bit is a pos, there are 0..9 = 10 possible bits and 10 < 16
    buttons: Vec<u16>,
}

fn solve_gf2_min_presses(mut matrix: Array2<u16>, mut target: Array1<u16>) -> Option<u32> {
    let (rows, cols) = matrix.dim();
    let mut pivot_row = 0;
    let mut col_to_pivot = vec![None; cols];

    for j in 0..cols {
        if pivot_row >= rows {
            break;
        }

        let mut p = pivot_row;
        while p < rows && matrix[[p, j]] == 0 {
            p += 1;
        }
        if p == rows {
            continue;
        }

        if p != pivot_row {
            for k in 0..cols {
                let tmp = matrix[[pivot_row, k]];
                matrix[[pivot_row, k]] = matrix[[p, k]];
                matrix[[p, k]] = tmp;
            }
            let tmp_t = target[pivot_row];
            target[pivot_row] = target[p];
            target[p] = tmp_t;
        }

        for i in 0..rows {
            if i != pivot_row && matrix[[i, j]] == 1 {
                for k in j..cols {
                    matrix[[i, k]] ^= matrix[[pivot_row, k]];
                }
                target[i] ^= target[pivot_row];
            }
        }

        col_to_pivot[j] = Some(pivot_row);
        pivot_row += 1;
    }

    for i in pivot_row..rows {
        if target[i] != 0 {
            return None;
        }
    }

    let mut free_cols = Vec::new();
    for (j, elem) in col_to_pivot.iter().enumerate().take(cols) {
        if elem.is_none() {
            free_cols.push(j);
        }
    }

    let mut min_presses = u32::MAX;
    let num_free = free_cols.len();

    for i in 0..(1 << num_free) {
        let mut x = vec![0u16; cols];

        for (bit_idx, &col_idx) in free_cols.iter().enumerate() {
            x[col_idx] = ((i >> bit_idx) & 1) as u16;
        }

        for j in (0..cols).rev() {
            if let Some(r) = col_to_pivot[j] {
                let mut sum = 0;
                for k in (j + 1)..cols {
                    sum ^= matrix[[r, k]] & x[k];
                }
                x[j] = target[r] ^ sum;
            }
        }

        let presses = x.iter().map(|&v| v as u32).sum();
        if presses < min_presses {
            min_presses = presses;
        }
    }

    if min_presses == u32::MAX {
        None
    } else {
        Some(min_presses)
    }
}

pub fn run() {
    let input = include_str!("../input/day10.txt");

    let parts: Vec<Machine> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let target_str = parts.next().unwrap();
            let target = target_str
                .chars()
                .skip(1)
                .take_while(|&ch| ch != ']')
                .enumerate()
                .fold(
                    0u16,
                    |acc, (i, ch)| {
                        if ch == '#' {
                            acc | (1 << i)
                        } else {
                            acc
                        }
                    },
                );

            let buttons = parts
                .take_while(|p| p.starts_with('('))
                .map(|p| {
                    p.trim_matches(|c| c == '(' || c == ')' || c == ',')
                        .split(',')
                        .map(|n| n.parse::<u16>().unwrap())
                        .fold(0u16, |acc, bit| acc | (1 << bit))
                })
                .collect();

            Machine { target, buttons }
        })
        .collect();

    let mut p1 = 0;
    for part in parts {
        let m = part.buttons.len();
        let n = 16; // u16 bits

        let mut matrix = Array2::<u16>::zeros((n, m));
        let mut target_arr = Array1::<u16>::zeros(n);
        for (i, row_idx) in (0..n).enumerate() {
            target_arr[i] = (part.target >> i) & 1;

            let mut row_vec = Vec::new();
            for button in &part.buttons {
                row_vec.push((button >> row_idx) & 1);
            }

            matrix.row_mut(i).assign(&Array1::from(row_vec));
        }

        p1 += solve_gf2_min_presses(matrix, target_arr).unwrap();
    }

    println!("Part 1: {p1}");
}

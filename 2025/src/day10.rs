use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};

type Matrix = [u32; 10];

fn solve_gs2(target_mask: u16, button_masks: &[u16]) -> usize {
    let num_buttons = button_masks.len();
    let num_lights = 10;
    let mut matrix: Matrix = [0; 10];
    let mut target_bits = target_mask;

    for (btn_idx, &mask) in button_masks.iter().enumerate() {
        for (row, elem) in matrix.iter_mut().enumerate().take(num_lights) {
            if (mask >> row) & 1 == 1 {
                *elem |= 1 << btn_idx;
            }
        }
    }

    let mut pivot_row = 0;
    let mut col_to_pivot = [None; 32];

    for (j, button) in col_to_pivot.iter_mut().enumerate().take(num_buttons) {
        if pivot_row >= num_lights {
            break;
        }
        let mut p = pivot_row;
        while p < num_lights && (matrix[p] >> j) & 1 == 0 {
            p += 1;
        }
        if p == num_lights {
            continue;
        }

        if p != pivot_row {
            matrix.swap(pivot_row, p);
            let bit_p = (target_bits >> p) & 1;
            let bit_pivot = (target_bits >> pivot_row) & 1;
            if bit_p != bit_pivot {
                target_bits ^= (1 << p) | (1 << pivot_row);
            }
        }

        for i in pivot_row + 1..num_lights {
            if i != pivot_row && (matrix[i] >> j) & 1 == 1 {
                matrix[i] ^= matrix[pivot_row];
                if (target_bits >> pivot_row) & 1 == 1 {
                    target_bits ^= 1 << i;
                }
            }
        }
        *button = Some(pivot_row);
        pivot_row += 1;
    }

    let mut free_vars = [0usize; 32];
    let mut num_free = 0;
    for (j, button) in col_to_pivot.iter().enumerate().take(num_buttons) {
        if button.is_none() {
            free_vars[num_free] = j;
            num_free += 1;
        }
    }

    (0..(1 << num_free))
        .map(|mask| {
            let mut x = 0u32;
            let mut presses = 0;
            for (k, &free_var) in free_vars.iter().enumerate().take(num_free) {
                if (mask >> k) & 1 == 1 {
                    x |= 1 << free_var;
                    presses += 1;
                }
            }
            for j in (0..num_buttons).rev() {
                if let Some(r) = col_to_pivot[j] {
                    let sum = (matrix[r] & x).count_ones() % 2;
                    let needed = ((target_bits >> r) & 1) as u32;
                    if needed ^ sum == 1 {
                        x |= 1 << j;
                        presses += 1;
                    }
                }
            }
            presses
        })
        .min()
        .unwrap()
}

fn solve_lp(target_joltages: &[f64], buttons_indices: &[Vec<usize>]) -> f64 {
    let mut vars = variables!();

    let btn_vars: Vec<_> = (0..buttons_indices.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let objective: Expression = btn_vars.iter().sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    for (row_idx, &target_val) in target_joltages.iter().enumerate() {
        let constraint_expr: Expression = buttons_indices
            .iter()
            .zip(&btn_vars)
            .filter(|(indices, _)| indices.contains(&row_idx)) // Solo si afecta a esta fila
            .map(|(_, &v)| v)
            .sum();

        problem.add_constraint(constraint_expr.eq(target_val));
    }

    let solution = problem.solve().unwrap();
    btn_vars.iter().map(|&v| solution.value(v)).sum()
}

pub fn run() {
    let input = include_str!("../input/day10.txt");

    let mut p1 = 0;
    let mut p2 = 0.0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split_whitespace();

        let tgt_str = parts.next().unwrap();
        let target_mask = tgt_str.chars().enumerate().fold(0u16, |acc, (i, c)| {
            if c == '#' {
                acc | (1 << (i - 1))
            } else {
                acc
            }
        });

        let mut buttons_p1 = Vec::new();
        let mut buttons_p2 = Vec::new();
        let mut target_joltages = Vec::new();

        for part in parts {
            if part.starts_with('(') {
                let nums: Vec<usize> = part
                    .trim_matches(|c| c == '(' || c == ')' || c == ',')
                    .split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect();

                let mask = nums.iter().fold(0u16, |acc, &bit| acc | (1 << bit));
                buttons_p1.push(mask);
                buttons_p2.push(nums);
            } else if part.starts_with('{') {
                target_joltages = part
                    .trim_matches(|c| c == '{' || c == '}' || c == ',')
                    .split(',')
                    .filter_map(|n| n.parse::<f64>().ok())
                    .collect();
            }
        }

        p1 += solve_gs2(target_mask, &buttons_p1);

        if !target_joltages.is_empty() {
            p2 += solve_lp(&target_joltages, &buttons_p2);
        }
    }

    println!("Part 1: {p1}, Part 2: {p2}");
}

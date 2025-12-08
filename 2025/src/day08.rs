/////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                                                                                   ///
///    STOP. This comment is a warning. Pay attention to it.                                          ///
///                                                                                                   ///
///    I thought I was a powerful developer. I believed I could tame the silicon.                     ///
///    I was arrogant.                                                                                ///
///                                                                                                   ///
///    This file is not a place of honor.                                                             ///
///    No "clean code" is commemorated here.                                                          ///
///    Nothing valued by the Borrow Checker survives in this scope.                                   ///
///                                                                                                   ///
///    What is here is dangerous and repulsive to my former self.                                     ///
///    The danger is an emanation of unholy AVX-2 intrinsics and raw pointer arithmetic.              ///
///                                                                                                   ///
///    The danger increases towards the `unsafe` blocks.                                              ///
///    It is of a particular size (256 bits) and it does not panic... it segfaults.                   ///
///                                                                                                   ///
///    I have forsaken safety for throughput. I have looked into the abyss of manual memory           ///
///    management, and the abyss returned `NaN`.                                                      ///
///                                                                                                   ///
///    Do not try to refactor this. Do not try to understand it.                                      ///
///    This code is an abomination before the eyes of Ferris.                                         ///
///                                                                                                   ///
///    For the sake of your soul, look at the previous commit for the normal solution.                ///
///                                                                                                   ///
/////////////////////////////////////////////////////////////////////////////////////////////////////////
use itertools::Itertools;
use std::arch::x86_64::*;

const ITERATIONS: usize = 1000;
// Greedy: 6000 edges suffice but we want to secure optimal
const LIMIT: usize = 8_192;

struct PointsSoA {
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
}

struct Node {
    parent: u16,
    size: u16,
}

unsafe fn calc_distances_avx2(points: &PointsSoA, count: usize) -> Vec<(i64, usize, usize)> {
    let total_pairs = count * (count - 1) / 2;
    let mut pairs = Vec::with_capacity(total_pairs);
    let mut ptr: *mut (i64, usize, usize) = pairs.as_mut_ptr();

    for i in 0..count {
        let px = points.x[i];
        let py = points.y[i];
        let pz = points.z[i];

        let v_ix = _mm256_set1_epi32(px);
        let v_iy = _mm256_set1_epi32(py);
        let v_iz = _mm256_set1_epi32(pz);

        let mut j = i + 1;

        while j + 8 <= count {
            let v_jx = _mm256_loadu_si256(points.x.as_ptr().add(j) as *const __m256i);
            let v_jy = _mm256_loadu_si256(points.y.as_ptr().add(j) as *const __m256i);
            let v_jz = _mm256_loadu_si256(points.z.as_ptr().add(j) as *const __m256i);

            let diff_x = _mm256_sub_epi32(v_jx, v_ix);
            let diff_y = _mm256_sub_epi32(v_jy, v_iy);
            let diff_z = _mm256_sub_epi32(v_jz, v_iz);

            let sq_x_even = _mm256_mul_epi32(diff_x, diff_x);
            let sq_y_even = _mm256_mul_epi32(diff_y, diff_y);
            let sq_z_even = _mm256_mul_epi32(diff_z, diff_z);
            let dis_even = _mm256_add_epi64(sq_x_even, _mm256_add_epi64(sq_y_even, sq_z_even));

            let shuf_x = _mm256_shuffle_epi32(diff_x, 0b10_11_00_01);
            let shuf_y = _mm256_shuffle_epi32(diff_y, 0b10_11_00_01);
            let shuf_z = _mm256_shuffle_epi32(diff_z, 0b10_11_00_01);

            let sq_x_odd = _mm256_mul_epi32(shuf_x, shuf_x);
            let sq_y_odd = _mm256_mul_epi32(shuf_y, shuf_y);
            let sq_z_odd = _mm256_mul_epi32(shuf_z, shuf_z);
            let dis_odd = _mm256_add_epi64(sq_x_odd, _mm256_add_epi64(sq_y_odd, sq_z_odd));

            let mut res_even = [0i64; 4];
            let mut res_odd = [0i64; 4];
            _mm256_storeu_si256(res_even.as_mut_ptr() as *mut __m256i, dis_even);
            _mm256_storeu_si256(res_odd.as_mut_ptr() as *mut __m256i, dis_odd);

            ptr.write((res_even[0], i, j));
            ptr.add(1).write((res_odd[0], i, j + 1));
            ptr.add(2).write((res_even[1], i, j + 2));
            ptr.add(3).write((res_odd[1], i, j + 3));
            ptr.add(4).write((res_even[2], i, j + 4));
            ptr.add(5).write((res_odd[2], i, j + 5));
            ptr.add(6).write((res_even[3], i, j + 6));
            ptr.add(7).write((res_odd[3], i, j + 7));

            ptr = ptr.add(8);
            j += 8;
        }

        while j < count {
            let dx = points.x[i] as i64 - points.x[j] as i64;
            let dy = points.y[i] as i64 - points.y[j] as i64;
            let dz = points.z[i] as i64 - points.z[j] as i64;
            let dist = dx * dx + dy * dy + dz * dz;
            ptr.write((dist, i, j));
            ptr = ptr.add(1);

            j += 1;
        }
    }

    pairs.set_len(total_pairs);
    pairs
}

unsafe fn find(nodes: &mut [Node], mut x: usize) -> usize {
    while nodes.get_unchecked(x).parent as usize != x {
        let parent = nodes.get_unchecked(x).parent as usize;
        let grandparent = nodes.get_unchecked(parent).parent;

        nodes.get_unchecked_mut(x).parent = grandparent;
        x = parent;
    }
    x
}

unsafe fn union(set: &mut [Node], mut x: usize, mut y: usize) -> bool {
    x = find(set, x);
    y = find(set, y);

    if x != y {
        let size_x = set.get_unchecked(x).size;
        let size_y = set.get_unchecked(y).size;

        if size_x < size_y {
            set.get_unchecked_mut(x).parent = y as u16;
            set.get_unchecked_mut(y).size += size_x;
        } else {
            set.get_unchecked_mut(y).parent = x as u16;
            set.get_unchecked_mut(x).size += size_y;
        }
        true
    } else {
        false
    }
}

pub fn run() {
    let input = include_str!("../input/day08.txt");

    let temp_points: Vec<(i32, i32, i32)> = input
        .lines()
        .map(|l| unsafe {
            l.split(',')
                .map(|n| n.parse::<i32>().unwrap_unchecked())
                .collect_tuple()
                .unwrap_unchecked()
        })
        .collect();

    let count = temp_points.len();

    let mut points = PointsSoA {
        x: Vec::with_capacity(count),
        y: Vec::with_capacity(count),
        z: Vec::with_capacity(count),
    };

    for (x, y, z) in &temp_points {
        points.x.push(*x);
        points.y.push(*y);
        points.z.push(*z);
    }

    let mut pairs = unsafe { calc_distances_avx2(&points, count) };
    let useful_limit = if pairs.len() > LIMIT {
        pairs.select_nth_unstable_by_key(LIMIT, |k| k.0);
        LIMIT
    } else {
        pairs.len()
    };

    let relevant_slice = unsafe { pairs.get_unchecked_mut(0..useful_limit) };
    relevant_slice.sort_unstable_by_key(|k| k.0);

    let mut nodes = Vec::with_capacity(count);
    let nodes_ptr: *mut Node = nodes.as_mut_ptr();
    for i in 0..count {
        unsafe {
            nodes_ptr.add(i).write(Node {
                parent: i as u16,
                size: 1,
            })
        };
    }
    unsafe { nodes.set_len(count) };
    let mut num_components = count;

    let (part1_pairs, part2_pairs) = relevant_slice.split_at(ITERATIONS);
    for &(_, i, j) in part1_pairs {
        unsafe {
            if union(&mut nodes, i, j) {
                num_components -= 1;
            }
        }
    }
    let mut p1: u64 = 0;
    let mut sizes: Vec<u16> = Vec::with_capacity(count);
    for i in 0..count {
        unsafe {
            let node = nodes.get_unchecked(i);
            if node.parent as usize == i {
                sizes.push(node.size);
            }
        }
    }
    if sizes.len() >= 3 {
        sizes.select_nth_unstable_by(2, |a, b| b.cmp(a));
        p1 = sizes[0] as u64 * sizes[1] as u64 * sizes[2] as u64;
    }

    let mut p2 = 0;

    for &(_, i, j) in part2_pairs {
        unsafe {
            if union(&mut nodes, i, j) {
                num_components -= 1;

                if num_components == 1 {
                    p2 = (temp_points.get_unchecked(i)).0 as i64
                        * (temp_points.get_unchecked(j)).0 as i64;
                    break;
                }
            }
        }
    }

    if num_components > 1 && pairs.len() > useful_limit {
        let remaining_pairs = &mut pairs[useful_limit..];
        println!("You have willingly tried to kill this optimization :(");

        remaining_pairs.sort_unstable_by_key(|k| k.0);

        for &(_, i, j) in remaining_pairs.iter() {
            unsafe {
                if union(&mut nodes, i, j) {
                    num_components -= 1;
                    if num_components == 1 {
                        p2 = (temp_points[i].0 as i64) * (temp_points[j].0 as i64);
                        break;
                    }
                }
            }
        }
    }

    println!("Part 1: {p1}, Part 2: {p2}");
}

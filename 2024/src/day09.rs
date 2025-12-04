
fn part1(files: Vec<(usize, i32)>, base: usize) -> usize {
    let mut files = files;
    let mut l = 0;
    let mut r = files.len() - 1;

    while l < r {
        while l < files.len() && files[l].1 != 0 {
            l += 1;
        }

        while r > 0 && files[r].1 == 0 {
            r -= 1;
        }

        if l < r {
            files.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    files
        .iter()
        .enumerate()
        .map(|(i, &(_, id))| (i + base) * id as usize)
        .sum()
}

fn part2(mut files: Vec<(usize, i32)>, base: usize) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == 0 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i].iter().position(|&(s, id)| id == 0 && size <= s) {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, 0);
            if size < s {
                files.insert(j + 1, (s - size, 0));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| (i + base) * id as usize)
        .sum()
}

pub fn run() {
    let input = include_str!("../input/day09.txt");
    let mut fs1 = Vec::new();
    let mut fs2 = Vec::new();
    let mut fid = 1;

    let first_elem = input.chars().next().and_then(|ch| ch.to_digit(10)).unwrap() as usize;

    for (i, ch) in input
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .enumerate()
        .skip(1)
    {
        let v = if i % 2 == 0 {
            fid += 1;
            fid - 1
        } else {
            0
        };
        let value = ch.to_digit(10).unwrap() as usize;
        fs1.extend((0..value).map(|_| (1, v)));
        fs2.push((value, v));
    }

    let p1 = part1(fs1, first_elem);
    let p2 = part2(fs2, first_elem);
    println!("Part 1: {p1}\nPart 2: {p2}");
}

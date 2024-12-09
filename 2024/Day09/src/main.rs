use std::fs;

fn solve(mut files: Vec<(usize, i32)>) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];
        if id == -1 {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id == -1 && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, -1);
            if size < s {
                files.insert(j + 1, (s - size, -1));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| if id == -1 { 0 } else { i * id as usize })
        .sum()
}

fn main() {
    let input = fs::read_to_string("./input").expect("File input not found");
    let mut fs1 = Vec::new();
    let mut fs2 = Vec::new();
    let mut fid = 0;
    for (i, ch) in input.chars().filter(|ch| ch.is_ascii_digit()).enumerate() {
        let v = if i % 2 == 0 {
            fid += 1;
            fid - 1
        } else {
            -1
        };
        let value = ch.to_digit(10).unwrap() as usize;
        fs1.extend((0..value).map(|_| (1, v)));
        fs2.push((value, v));
    }
    let p1 = solve(fs1);
    let p2 = solve(fs2);
    println!("Part 1: {p1}\nPart 2: {p2}");
}

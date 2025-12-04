
/// There are several ways to improve efficiency, as I am not concerned
/// about that in this simple problem, I will leave it like this which
/// I consider to be the most readable.
static WIN: [(char, char); 3] = [('A', 'Y'), ('B', 'Z'), ('C', 'X')];
static DRAW: [(char, char); 3] = [('A', 'X'), ('B', 'Y'), ('C', 'Z')];
static X: u32 = 1; // Rock
static Y: u32 = 2; // Paper
static Z: u32 = 3; // Scissors

pub fn run() {
    let input = include_str!("../input/day02.txt");
    let games: Vec<(char, char)> = input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            (chars[0], chars[2])
        })
        .collect();

    let (mut p1, mut p2) = (0, 0);
    for game in &games {
        match game.1 {
            'X' => {
                // Lose
                p2 += match game.0 {
                    'A' => Z,
                    'B' => X,
                    'C' => Y,
                    _ => unreachable!(),
                }
            }
            'Y' => {
                // Draw
                p2 += match game.0 {
                    'A' => X,
                    'B' => Y,
                    'C' => Z,
                    _ => unreachable!(),
                };
                p2 += 3;
            }
            'Z' => {
                p2 += match game.0 {
                    'A' => Y,
                    'B' => Z,
                    'C' => X,
                    _ => unreachable!(),
                };
                p2 += 6;
            }
            _ => unreachable!(),
        }
        if WIN.contains(&(game.0, game.1)) {
            p1 += 6;
        } else if DRAW.contains(&(game.0, game.1)) {
            p1 += 3;
        }
        p1 += match game.1 {
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            _ => unreachable!(),
        };
    }

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

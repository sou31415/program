use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
#[derive(Clone, Copy, Debug)]
struct place {
    x: usize,
    y: usize,
}
fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut score: usize = 0;
    let mut p = place { x: 0, y: 0 };
    let mut g: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if let Ok(x) = s[i][j].to_string().parse::<usize>() {
                g[i][j] = x;
            } else if s[i][j] == '.' {
                g[i][j] = 0;
            } else if s[i][j] == '@' {
                p.x = j;
                p.y = i;
            }
        }
    }
    println!("{:?}", p);
    let mut t: usize = 0;
    while t < 4 {
        let mut p2: (usize, usize) = (0, 0);
        let mut d: usize = 0;
        if p.x > 0 {
            if d <= g[p.y][p.x - 1] {
                d = g[p.y][p.x - 1];
                p2 = (p.y, p.x - 1);
            }
        }
        if p.x < w - 1 {
            if d <= g[p.y][p.x + 1] {
                d = max(d, g[p.y][p.x + 1]);
                p2 = (p.y, p.x + 1);
            }
        }
        if p.y > 0 {
            if d <= g[p.y - 1][p.x] {
                d = max(d, g[p.y - 1][p.x]);
                p2 = (p.y - 1, p.x);
            }
        }
        if p.y < h - 1 {
            if d <= g[p.y + 1][p.x] {
                d = max(d, g[p.y + 1][p.x]);
                p2 = (p.y + 1, p.x);
            }
        }
        score += d;
        g[p2.0][p2.1] = 0;
        p.x = p2.1;
        p.y = p2.0;
        t += 1;
        println!("{:?}", p);
    }
    println!("{}", score);
}

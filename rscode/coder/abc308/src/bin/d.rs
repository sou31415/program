#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
#[fastout]
fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut uf = UnionFind::new(h * w);

    let mut g: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'n' {
                g[i][j] = 1;
            } else if s[i][j] == 'u' {
                g[i][j] = 2;
            } else if s[i][j] == 'k' {
                g[i][j] = 3;
            } else if s[i][j] == 'e' {
                g[i][j] = 4;
            } else if s[i][j] != 's' {
                g[i][j] = std::usize::MAX;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if i < h - 1 {
                if g[i][j] > g[i + 1][j] {
                    if g[i][j] - g[i + 1][j] == 1 || (g[i][j] == 4 && g[i + 1][j] == 0) {
                        uf.union(i * w + j, (i + 1) * w + j);
                    }
                } else if g[i][j] < g[i + 1][j] {
                    if g[i + 1][j] - g[i][j] == 1 || (g[i][j] == 0 && g[i + 1][j] == 4) {
                        uf.union(i * w + j, (i + 1) * w + j);
                    }
                }
            }
            if i > 0 {
                if g[i][j] > g[i - 1][j] {
                    if g[i][j] - g[i - 1][j] == 1 || (g[i][j] == 4 && g[i - 1][j] == 0) {
                        uf.union(i * w + j, (i - 1) * w + j);
                    }
                } else if g[i - 1][j] > g[i][j] {
                    if g[i - 1][j] - g[i][j] == 1 && (g[i - 1][j] == 4 && g[i][j] == 0) {
                        uf.union(i * w + j, (i - 1) * w + j);
                    }
                }
            }
            if j < w - 1 {
                if g[i][j] > g[i][j + 1] {
                    if g[i][j] - g[i][j + 1] == 1 || (g[i][j] == 4 && g[i][j + 1] == 0) {
                        uf.union(i * w + j, i * w + j + 1);
                    }
                } else if g[i][j] < g[i][j + 1] {
                    if g[i][j + 1] - g[i][j] == 1 || (g[i][j] == 0 && g[i][j + 1] == 4) {
                        uf.union(i * w + j, i * w + 1 + j);
                    }
                }
            }
            if j != 0 {
                if g[i][j] > g[i][j - 1] {
                    if g[i][j] - g[i][j - 1] == 1 || g[i][j] == 4 && g[i][j - 1] == 0 {
                        uf.union(i * w + j, i * w + j - 1);
                    }
                } else if g[i][j - 1] > g[i][j] {
                    if g[i][j - 1] - g[i][j] == 1 && g[i][j - 1] == 4 && g[i][j] == 0 {
                        uf.union(i * w + j, i * w + j - 1);
                    }
                }
            }
        }
    }
    println!("{}", if uf.equiv(0, h * w - 1) { "Yes" } else { "No" });
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}

pub fn power(n: usize, mut x: usize) -> usize {
    let mut a: usize = 1;
    let mut b: usize = n;
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            a *= b;
            b *= b;
            x -= 1usize << i;
        } else {
            b *= b;
        }
        i += 1;
    }
    a
}
pub fn powm(n: usize, m: usize, mut x: usize) -> usize {
    let mut b: usize = n;
    let mut a: usize = 1;
    let mut i = 0;
    while x != 0 {
        if 1_usize << i & x != 0 {
            a = (a * b) % m;
            b = (b * b) % m;
            x ^= 1_usize << i;
        } else {
            b = (b * b) % m;
        }
        i += 1;
    }
    a
}
pub fn rt(n: usize) -> usize {
    let mut l: u128 = 1;
    let mut r: u128 = n as u128;
    while r - l > 1 {
        let m = (r + l) / 2;
        if m * m == n as u128 {
            return m as usize;
        }
        if m * m > n as u128 {
            r = m;
        } else {
            l = m;
        }
    }
    return l as usize;
}

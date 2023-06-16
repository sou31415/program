#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        h:usize,w:usize,
        a:[Chars;h],
    }
    let mut c: usize = 0;
    let mut g: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut m: usize = min(h, w);
    let mut v: Vec<usize> = vec![0; m];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' && !g[i][j] {
                g[i][j] = true;
                c = 1;
                for k in 1..min(h - i, w - j) {
                    if !g[i + k][j + k] && a[i + k][j + k] == '#' {
                        c += 1;
                    } else {
                        break;
                    }
                    g[i + k][j + k] = true;
                }
            }
            if c % 2 == 1 && c > 1 {
                v[(c / 2) - 1] += 1;
            }
            c = 0;
        }
    }
    println!("{}", v.iter().join(" "));
}

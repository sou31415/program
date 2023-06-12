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
        s:[Chars;h],
    }
    let mut cnt: usize = 0;
    let mut c: bool = false;
    let mut v: Vec<usize> = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                cnt += 1;
            }
        }
        c = false;
        v[i] = cnt;
        cnt = 0;
    }
    for i in 0..(h - 1) {
        if v[i] + 1 == v[i + 1] && v[i] != 0 {
            for j in 0..w {
                if s[i][j] == '.' && s[i + 1][j] == '#' {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
            }
        }
    }
    for i in 0..(h - 1) {
        if v[i] == v[i + 1] + 1 && v[i] != 0 {
            for j in 0..w {
                if s[i + 1][j] == '.' && s[i][j] == '#' {
                    println!("{} {}", i + 2, j + 1);
                    return;
                }
            }
        }
    }
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}

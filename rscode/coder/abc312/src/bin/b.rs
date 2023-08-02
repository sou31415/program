#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
use superslice::Ext;
#[fastout]
fn main() {
    let yes = String::from("Yes");
    let no = String::from("No");
    input! {
        n:usize,m:usize,
        s:[Chars;n],
    }
    let mut f = false;
    let mut v: Vec<(usize, usize)> = vec![];
    for i in 0..(n - 8) {
        for j in 0..(m - 8) {
            for k in i..(i + 3) {
                for l in j..(j + 3) {
                    if s[k][l] == '.' {
                        f = true;
                    }
                }
            }
            for k in (i + 6)..(i + 9) {
                for l in (j + 6)..(j + 9) {
                    if s[k][l] == '.' {
                        f = true;
                    }
                }
            }
            for k in 0..4 {
                if s[i + 3][j + k] == '#' {
                    f = true;
                }
            }
            for k in 0..4 {
                if s[i + k][j + 3] == '#' {
                    f = true;
                }
            }
            for k in 0..4 {
                if s[i + 5][j + 5 + k] == '#' {
                    f = true;
                }
            }
            for k in 0..4 {
                if s[i + 5 + k][j + 5] == '#' {
                    f = true;
                }
            }
            if !f {
                v.push((i, j));
            }
            f = false;
        }
    }
    for i in 0..v.len() {
        println!("{} {}", v[i].0 + 1, v[i].1 + 1);
    }
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}

pub fn matrix_pow(mut r: Vec<Vec<usize>>, m: usize, mut x: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = vec![vec![0; r.len()]; r.len()];
    for i in 0..r.len() {
        v[i][i] = 1;
    }
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            let mut d: Vec<Vec<usize>> = vec![vec![0, 0], vec![0, 0]];
            for i in 0..r.len() {
                for j in 0..r.len() {
                    for k in 0..r.len() {
                        d[i][j] += v[i][k] * r[k][j];
                        d[i][j] %= m;
                    }
                }
            }
            x -= 1usize << i;
            v = d;
        }
        let mut d: Vec<Vec<usize>> = vec![vec![0, 0], vec![0, 0]];
        for i in 0..r.len() {
            for k in 0..r.len() {
                for j in 0..r.len() {
                    d[i][j] += r[i][k] * r[k][j];
                    d[i][j] %= m;
                }
            }
        }
        r = d;
        i += 1;
    }
    return v;
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
        if 1usize << i & x != 0 {
            a = (a * b) % m;
            b = (b * b) % m;
            x ^= 1usize << i;
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

pub fn rotate(s: String) -> bool {
    let q = s.chars().collect::<Vec<char>>();
    let n = q.len();
    for i in 0..(n / 2) {
        if q[i] != q[n - i - 1] {
            return false;
        }
    }
    return true;
}

pub fn rotate_diff(q: String) -> usize {
    let s = q.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut result: usize = 0;
    for i in 0..(n / 2) {
        if s[i] != s[n - i - 1] {
            result += 1;
        }
    }
    result
}

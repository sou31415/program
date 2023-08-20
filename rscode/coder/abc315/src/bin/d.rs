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
        h:usize,w:usize,
        s:[Chars;h]
    }
    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            set.insert(s[i][j]);
        }
    }
    if set.len() == 1 {
        println!("0");
        return;
    }
    let mut d: bool = false;
    let mut f = false;
    let mut v: Vec<bool> = vec![false; h];
    let mut v1: Vec<bool> = vec![false; w];
    loop {
        for i in 0..h {
            for j in 0..w - 1 {
                if s[i][j] != s[i][j + 1] && !v[i] {
                    f = true;
                    d = true;
                    break;
                }
            }
            if !f {
                v[i] = true;
            }
            f = false;
        }
        for j in 0..w {
            for i in 0..h - 1 {
                if s[i][j] != s[i + 1][j] && !v1[j] {
                    f = true;
                    d = true;
                    break;
                }
            }
            if !f {
                v1[j] = true;
            }
            f = false;
        }
        if d {
            break;
        }
        d = false;
    }
    let mut cnt: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if !v[i] && !v1[j] {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
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

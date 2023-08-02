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
        mut a:[usize;n],
        mut b:[usize;m]
    }
    let mut cnt: usize = std::usize::MAX;
    let mut r = a.clone();
    let mut l = b.clone();
    r.sort();
    r.dedup();
    l.sort();
    l.dedup();
    let mut x = vec![0usize; r.len()];
    let mut y = vec![0usize; l.len()];
    for i in 0..n {
        if let Ok(s) = r.binary_search(&a[i]) {
            x[s] += 1;
        }
    }
    for i in 1..x.len() {
        x[i] += x[i - 1];
    }
    for i in 0..m {
        if let Ok(s) = l.binary_search(&b[i]) {
            y[s] += 1;
        }
    }
    for i in (0..y.len() - 1).rev() {
        y[i] += y[i + 1];
    }
    for i in 0..y.len() {
        if let Ok(d) = x.binary_search(&y[i]) {
            println!("{} a", max(r[d], l[i]));
            return;
        } else if let Err(d) = x.binary_search(&y[i]) {
            if d == x.len() {
                if x[d - 1] >= y[i] {
                    cnt = min(cnt, x[d - 1] + 1);
                }
            } else {
                if x[d] >= y[i] {
                    cnt = min(cnt, min(r[d], l[i]));
                }
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

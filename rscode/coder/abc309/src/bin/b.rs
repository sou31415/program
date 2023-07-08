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
        n:usize,s:[Chars;n],
    }
    let mut r = s.clone();
    for i in 0..n - 1 {
        r[0][i + 1] = s[0][i];
    }
    for i in 0..n - 1 {
        r[i + 1][n - 1] = s[i][n - 1];
    }
    for i in 0..n - 1 {
        r[n - 1][i] = s[n - 1][i + 1];
    }
    for i in 0..n - 1 {
        r[i][0] = s[i + 1][0];
    }
    for i in 0..n {
        println!("{}", r[i].iter().join(""));
    }
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
pub fn powm(n: usize, m: usize, c: usize) -> usize {
    let k: usize = 1;
    let mut x = c;
    let mut b: usize = n;
    let mut a: usize = 1;
    let mut i = 0;
    while x != 0 {
        if k << i & x != 0 {
            a = (a * b) % m;
            b = (b * b) % m;
            x ^= k << i;
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

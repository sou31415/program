#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
#[fastout]
fn main() {
    input! {
        n:usize,s:Chars,
    }
    let mut set: VecDeque<usize> = VecDeque::new();
    let mut set2: VecDeque<usize> = VecDeque::new();
    for i in 0..s.len() {
        if s[i] == '(' {
            set.push_back(i);
        } else if s[i] == ')' {
            set2.push_back(i);
        }
    }
    if set[0] == 0 {
        set.pop_front();
    }
    let len = set.len();
    let len2 = set2.len();
    if set[len - 1] == len - 1 {
        set.pop_back();
    }
    if set2[0] == 0 {
        set.pop_front();
    }
    if set2[len - 1] = s.len() - 1 {
        set.pop_back();
    }
    for i in 0..s.len() {
        if i < set[0] || i > set2[len - 1] {
            print!("{}", s[i]);
        }
    }
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}

pub fn power(n: usize, r: usize) -> usize {
    let k: usize = 1;
    let mut x: usize = r;
    let mut a: usize = 1;
    let mut b: usize = n;
    let mut i: usize = 0;
    while x != 0 {
        if k << i & x != 0 {
            a *= b;
            b *= b;
            x -= k << i;
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
    let mut m: u128 = 0;
    while r - l > 1 {
        m = (r + l) / 2;
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

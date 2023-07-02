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
        n:usize,m:usize,
        c:[String;n],
        d:[String;m],
        p:[usize;m+1]
    }

    let set = d.iter().collect::<HashSet<&String>>();
    let mut result: usize = 0;
    for i in 0..n {
        if !set.contains(&&c[i]) {
            result += p[0];
        } else {
            for j in 0..m {
                if d[j] == c[i] {
                    result += p[j + 1];
                }
            }
        }
    }
    println!("{}", result);
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

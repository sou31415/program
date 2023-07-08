#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::algo::{toposort, DfsSpace};
use petgraph::graph::DiGraph;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,
        p:[Usize1;n-1],
        xy:[(Usize1,usize);m],
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n]; //親子関係を表すグラフ
    for i in 0..n - 1 {
        g[p[i]].push(i);
    }
    let mut topo = vec![];
    for i in 0..g.len() {
        if !g[i].is_empty() {
            topo.push((i, g[i][0]));
        }
    }
    let g: DiGraph<(), ()> = DiGraph::from_edges(&topo);
    let mut space = DfsSpace::new(&g);
    let result = topopsort(&g, Some(&mut space));
    let exnode = result
        .unwrap()
        .iter()
        .map(|x| (x.index()))
        .collect::<Vec<_>>();
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

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
        n:usize,a:[Usize1;3*n],
    }
    let mut time: Vec<usize> = vec![0; n];
    let mut f: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut set: HashSet<usize> = (0..n).collect();
    for i in 0..3 * n {
        time[a[i]] += 1;
        if time[a[i]] == 2 {
            time[a[i]] += 1;
            f[a[i]] = (a[i], i);
        }
    }
    f.sort_by_key(|x| x.1);
    println!("{}", f.iter().map(|&x| x.0 + 1).join(" "));
}
/*
pub fn ziparam(a:usize,b:usize) -> usize{ // |a:usize - b:usize| -> usize
    return max(a,b) - min(a,b);
}

pub fn power(n:usize,x:usize) -> usize{ // n ^ x -> O(logx)
    let k: usize = 1;
    let mut b: usize = n;
    let mut a: usize = 1;
    while x != 0{
        if k << i & x != 0 {
            a *= b;
            b *= b;
            x ^= (k << i);
        } else {
            b *= b;
        }
    }
    a
}

pub fn powm(n: usize, c: usize, m: usize) -> usize { // n ^ c mod m -> O(logc)
    let k: usize = 1;
    let mut x = c;
    let mut b: usize = n;
    let mut a: usize = 1;
    let mut i = 0;
    while x != 0 {
        if k << i & x != 0 {
            a = (a * b) % m;
            b = (b * b) % m;
            x ^= (k << i);
        } else {
            b = (b * b) % m;
        }
        i += 1;
    }
    a
}

*/

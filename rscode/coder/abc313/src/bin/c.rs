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
        n:usize,mut a:[usize;n]
    }
    a.sort();
    if n == 1 {
        println!("0");
        return;
    }
    let mut d: usize = 0;
    let mut e: usize = 0;
    let mut s: usize = a.iter().sum();
    let mut avg = s / n;
    let mut m: usize = s % n;
    let mut r = 0;
    let mut l = 0;
    for i in 0..n {
        if a[i] < avg {
            l += ziparam(a[i], avg);
            d += 1;
        } else {
            if e < m {
                r += ziparam(a[i], avg + 1);
                e += 1;
            } else {
                r += ziparam(a[i], avg);
                e += 1;
            }
        }
    }
    println!("{}", max(l, r));
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}

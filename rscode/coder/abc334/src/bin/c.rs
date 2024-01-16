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
        n:usize,k:usize,a:[Usize1;k]
    }
    if k == 1 {
        println!("0");
        return;
    }
    if k == n {
        println!("{}", n / 2);
        return;
    }
    let mut result: usize = 0;
    let mut i: usize = 0;
    loop {
        result += a[i + 1] - a[i];
        i += 2;
        if a.len() - 1 <= i {
            break;
        }
    }
    if k % 2 == 0 {
        println!("{}", result);
        return;
    }
    i = 1;
    let mut result2: usize = 0;
    loop {
        result2 += a[i + 1] - a[i];
        i += 2;
        if a.len() - 1 <= i {
            break;
        }
    }
    let mut v = a.clone();
    v.reverse();
    let mut result3: usize = std::usize::MAX;
    i = 0;
    if a.len() > 4 {
        result3 = 0;
        loop {
            result3 += a[i + 1] - a[i];
            result3 += v[i] - v[i + 1];
            i += 2;
            if v.len() / 2 == i {
                break;
            }
        }
    }
    println!("{}", min(result, min(result2, result3)));
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}

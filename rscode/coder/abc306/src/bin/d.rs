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
        n:usize,
        x:usize,y:isize,
    }
    let mut i: usize = 1;
    let mut dp: Vec<Vec<isize>> = vec![vec![-10000000000; n]; 2];
    if x == 0 {
        dp[0][0] = max(0, y);
        dp[1][0] = 0;
    } else {
        dp[0][0] = 0;
        dp[1][0] = max(0, y);
    }
    input! {
        ab:[(usize,isize);n-1]
    }
    for (a, b) in ab {
        if a == 0 {
            dp[0][i] = max(dp[0][i - 1] + b, max(dp[1][i - 1] + b, dp[0][i - 1]));
            dp[1][i] = dp[1][i - 1];
        } else if a == 1 {
            dp[1][i] = max(dp[1][i - 1], dp[0][i - 1] + b);
            dp[0][i] = dp[0][i - 1];
        }
        i += 1;
    }
    println!("{}", max(dp[1][n - 1], dp[0][n - 1]));
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

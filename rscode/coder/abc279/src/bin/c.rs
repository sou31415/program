#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize , w:usize,
        s:[Chars;h],t:[Chars;h]
    }
    let mut count2: Vec<Vec<char>> = vec![vec![]; w];
    let mut count1: Vec<Vec<char>> = vec![vec![]; w];
    for i in 0..h {
        for j in 0..w {
            count2[j].push(s[i][j]);
            count1[j].push(t[i][j]);
        }
    }
    count2.sort();
    count1.sort();
    println!("{}", if count2 == count1 { "Yes" } else { "No" });
}

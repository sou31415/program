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
        ab:[[usize;w];h],
    }
    let mut count:usize = 0;
    let mut dp:Vec<Vec<usize>> = vec![vec![0;w];h];
    dp[0] = vec![1_usize;w];
    for i in 0..h{
        dp[i][0] = 1;
    }
    println!("{}",count);
}

fn search(v:&Vec<Vec<usize>> )

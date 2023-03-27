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

    println!("{}",count);
}

fn dfs(v:Vec<Vec<usize>> , count:&mut usize ,h:usize , w:usize) {


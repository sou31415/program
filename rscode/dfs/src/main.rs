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
    let count = dfs(h, w, ab);
    println!("{}", count);
}

fn dfs2(n: usize, g: &mut Vec<Vec<usize>>) -> Vec<bool> {
    let mut seen: Vec<bool> = vec![false; n];
    let mut stack: Vec<usize> = vec![];
    seen[0] = true;
    while let Some(x) = g[0].pop() {
        stack.push(x);
        seen[x] = true;
    }
    while let Some(x) = stack.pop() {
        while let Some(y) = g[x].pop() {
            stack.push(y);
            seen[y] = true;
        }
    }
    seen
}

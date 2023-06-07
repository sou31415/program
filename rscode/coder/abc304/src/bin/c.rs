#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        n:usize,d:usize,
        xy:[(isize,isize);n]
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let q = (xy[i].0 - xy[j].0).abs() * (xy[i].0 - xy[j].0).abs()
                + (xy[i].1 - xy[j].1).abs() * (xy[i].1 - xy[j].1).abs();
            if d * d >= q as usize {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let v = dfs(n, &mut g);
    for i in 0..n {
        if v[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(n: usize, g: &mut Vec<Vec<usize>>) -> Vec<bool> {
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

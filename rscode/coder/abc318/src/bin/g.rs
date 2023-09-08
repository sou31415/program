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
        n:usize,m:usize,
        a:Usize1,b:Usize1,c:Usize1,
        uv:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (a, b) in uv {
        g[a].push(b);
        g[b].push(a);
        uf.union(a, b);
    }
    if uf.equiv(a, b) && uf.equiv(b, c) {
        println!("No");
        return;
    }
    let f = dfs(n, a, b, c, g);
    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(n: usize, a: usize, b: usize, c: usize, mut g: Vec<Vec<usize>>) -> bool {
    let mut stack = vec![];
    while let Some(x) = g[a].pop() {
        if x == b {
            stack.push((x, true));
        } else {
            stack.push((x, false));
        }
    }
    while let Some((i, j)) = stack.pop() {
        while let Some(y) = g[i].pop() {
            if j && y == c {
                return true;
            } else if !j && y == b {
                stack.push((y, true));
                continue;
            } else if !j && y == c {
                continue;
            } else if y == a {
                continue;
            }
            stack.push((y, j));
        }
    }
    return false;
}

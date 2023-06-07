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

fn dfs(h: usize, w: usize, g: Vec<Vec<usize>>) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut count: usize = 0;
    let mut stack: Vec<(usize, usize, HashSet<usize>)> = vec![];
    set.insert(g[0][0]);
    stack.push((0, 0, set.clone()));
    while let Some((x, y, mut set)) = stack.pop() {
        if x < (w - 1) && y < h {
            let mut sets = set.clone();
            sets.insert(g[y][x + 1]);
            stack.push((x + 1, y, sets.clone()));
        }
        if y < (h - 1) && x < w {
            let mut sets = set.clone();
            sets.insert(g[y + 1][x]);
            stack.push((x, y + 1, sets.clone()));
        }
        if x == w - 1 && y == h - 1 {
            set.insert(g[h - 1][w - 1]);
            if set.len() == h + w - 1 {
                count += 1;
            }
            set.clear();
        }
    }
    count
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

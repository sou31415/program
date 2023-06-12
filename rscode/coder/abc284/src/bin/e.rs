use proconio::{input, marker::Usize1};
use std::collections::HashSet;
fn main() {
    input! {
        n:usize , m:usize,
        ab :[(Usize1 , Usize1);m],
    }
    let mut node: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        node[a].push(b);
        node[b].push(a);
    }
    if node[0].is_empty() {
        println!("1");
        return;
    }
    let result: usize = dfs(n, &mut node);
    println!("{}", result);
}

fn dfs(n: usize, v: &mut Vec<Vec<usize>>) -> usize {
    let mut r: usize = 1;
    let mut path: HashSet<usize> = HashSet::new();
    let mut stack: Vec<usize> = vec![];
    let mut seen: Vec<bool> = vec![false; n];
    for i in 0..v[0].len() {
        stack.push(v[0][i]);
        r += 1;
    }
    path.insert(0);
    seen[0] = true;
    while let Some(x) = stack.pop() {
        if r >= 100000 {
            return 100000_usize;
        }
        for i in 0..v[x].len() {
            if !seen[v[x][i]] {
                stack.push(v[x][i]);
                r += 1;
            }
        }
        seen[x] = true;
        if v[x].iter().all(|&k| seen[k]) {
            seen[x] = false;
        }
    }
    return r;
}

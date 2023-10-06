use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
    n:usize,d:usize,
        ab:[(isize,isize);n]

              }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if abser(ab[j], ab[i]) <= (d * d) as isize {
                g[j].push(i);
                g[i].push(j);
            }
        }
    }
    let dis = dfs(n, g);
    for i in 0..n {
        if dis[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
fn abser(x: (isize, isize), y: (isize, isize)) -> isize {
    (x.0 - y.0) * (x.0 - y.0) + (x.1 - y.1) * (x.1 - y.1)
}
fn dfs(n: usize, mut g: Vec<Vec<usize>>) -> Vec<bool> {
    let mut seen: Vec<bool> = vec![false; n];
    let mut stack: Vec<usize> = vec![];
    while let Some(x) = g[0].pop() {
        stack.push(x);
    }
    seen[0] = true;
    while let Some(x) = stack.pop() {
        while let Some(y) = g[x].pop() {
            stack.push(y);
        }
        seen[x] = true;
    }
    seen
}

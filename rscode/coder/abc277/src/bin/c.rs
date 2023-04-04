use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1 , Usize1);n],
    }
    let mut v: Vec<usize> = vec![]; //座標圧縮用
    v.push(0);
    for &(a, b) in &ab {
        v.push(a);
        v.push(b);
    }
    v.sort();
    v.dedup();
    let mut g: Vec<Vec<usize>> = vec![vec![]; v.len()];
    for (a, b) in ab {
        let l: usize = v.binary_search(&a).unwrap();
        let r: usize = v.binary_search(&b).unwrap();
        g[l].push(r);
        g[r].push(l);
    }
    let decoy: usize = dfs(g, v);
    println!("{}", decoy + 1);
}

fn dfs(g: Vec<Vec<usize>>, v: Vec<usize>) -> usize {
    let mut seen: Vec<bool> = vec![false; g.len()];
    if g[0].is_empty() {
        return 0_usize;
    }
    let mut stack = vec![];
    let mut graph = g.clone();
    while let Some(x) = graph[0].pop() {
        stack.push(x);
    }
    seen[0] = true;
    while let Some(x) = stack.pop() {
        while let Some(y) = graph[x].pop() {
            stack.push(y);
        }
        seen[x] = true;
    }
    let mut result: usize = 0;
    for i in 0..seen.len() {
        if seen[i] {
            result = i;
        }
    }
    v[result]
}

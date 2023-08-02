use proconio::input;
fn main() {
    input! {
    n:usize,ab:[(usize,usize);n]
          }
    let mut v = vec![1usize];
    for (a, b) in ab.iter().map(|&x| x) {
        v.push(a);
        v.push(b);
    }
    v.sort();
    v.dedup();
    let mut g: Vec<Vec<usize>> = vec![vec![]; v.len()];
    for (a, b) in ab {
        let l = v.binary_search(&a).unwrap();
        let r = v.binary_search(&b).unwrap();
        g[l].push(r);
        g[r].push(l);
    }
    let ans = dfs(v.len(), &mut g);
    println!("{}", v[ans]);
}

fn dfs(n: usize, g: &mut Vec<Vec<usize>>) -> usize {
    let mut stack = vec![];
    let mut seen = vec![false; n];
    while let Some(x) = g[0].pop() {
        stack.push(x);
    }
    seen[0] = true;
    while let Some(x) = stack.pop() {
        while let Some(y) = g[x].pop() {
            if !seen[y] {
                stack.push(y);
            }
        }
        seen[x] = true;
    }
    for i in (0..n).rev() {
        if seen[i] {
            return i;
        }
    }
    return 0;
}

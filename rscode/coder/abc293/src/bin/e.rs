#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        a:usize,x:usize,mut m:usize,
    }
    let k = power(a, m, x);
    let result = f(k, m);
    println!("{}", result);
}

fn f(v: Vec<Vec<usize>>, m: usize) -> usize {
    let decoy: usize = v[0][1] % m;
    decoy
}

fn power(a: usize, m: usize, mut x: usize) -> Vec<Vec<usize>> {
    let mut r = vec![vec![a, 1], vec![0, 1]];
    let mut v: Vec<Vec<usize>> = vec![vec![1, 0], vec![0, 1]];
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            let mut d: Vec<Vec<usize>> = vec![vec![0, 0], vec![0, 0]];
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        d[i][j] += v[i][k] * r[k][j];
                        d[i][j] %= m;
                    }
                }
            }
            x -= 1usize << i;
            v = d;
        }
        let mut d: Vec<Vec<usize>> = vec![vec![0, 0], vec![0, 0]];
        for i in 0..2 {
            for k in 0..2 {
                for j in 0..2 {
                    d[i][j] += r[i][k] * r[k][j];
                    d[i][j] %= m;
                }
            }
        }
        r = d;
        i += 1;
    }
    return v;
}

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
    let r: Vec<Vec<usize>> = vec![vec![a, 1], vec![0, 1]];
    let k = matrix_pow(r, a, m, x);
    let result = k[0][1];
    println!("{}", result);
}
pub fn matrix_pow(mut r: Vec<Vec<usize>>, a: usize, m: usize, mut x: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = vec![vec![0; r.len()]; r.len()];
    for i in 0..r.len() {
        v[i][i] = 1;
    }
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

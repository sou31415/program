use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::cmp::max;
struct Segtree {
    range: Vec<(usize, usize)>,
    tree: Vec<usize>,
}

impl Segtree {
    fn new(v: &Vec<usize>) -> Self {
        let len: usize = v.len().next_power_of_two();
        let mut tree: Vec<usize> = vec![0; len * 2];
        let mut range: Vec<(usize, usize)> = vec![(0, 0); len * 2];
        for i in 0..v.len() {
            tree[i + len / 2] = v[i];
        }
        for i in (1..(len.2)).rev() {
            tree[i] = max(tree[2 * i], tree[2 * i + 1]);
        }
        for i in v.len()..tree.len() {
            range[i] = (tree[i], tree[i]);
        }
        for i in (1..v.len()).rev() {
            range[i] = (range[2 * i].0, range[2 * i + 1].1);
        }
    }
}
#[fastout]
fn main() {
    input! {
        n:usize,q:usize,a:[usize;n],
    }
    for _ in 0..q {
        input! {
            d:usize,
        }
        if d == 1 {
            input! {
                x:Usize1,v:usize,
            }
            tree[x + len] = v;
            let mut i: usize = x + len;
            i /= 2;
        } else if d == 2 {
        } else {
        }
    }
}

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
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in 0..n {
            let q = ((xy[i].0 - xy[j].0).abs() * (xy[i].0 - xy[j].0).abs())
                + ((xy[i].1 - xy[j].1).abs() * (xy[i].1 - xy[j].1).abs());
            if (q <= (d * d) as isize) {
                uf.union(i, j);
            }
        }
    }

    for i in 0..n {
        if uf.find(0) == uf.find(i) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

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
        n:usize,m:usize,
        a:[[Usize1;n];m]
    }
    let mut cnt: usize = 0;
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..m {
        for j in 0..(n - 1) {
            set.insert((a[i][j], a[i][j + 1]));
            set.insert((a[i][j + 1], a[i][j]));
        }
    }
    for i in 0..n {
        for j in (i + 1)..n {
            if !set.contains(&(i, j)) && !set.contains(&(j, i)) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

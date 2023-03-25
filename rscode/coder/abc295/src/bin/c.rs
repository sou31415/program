#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut cnt: usize = 0;
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..n {
        if set.contains(&a[i]) {
            cnt += 1;
            set.remove(&a[i]);
        } else {
            set.insert(a[i]);
        }
    }
    println!("{}", cnt);
}

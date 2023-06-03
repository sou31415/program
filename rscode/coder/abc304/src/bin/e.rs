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
        n:usize,m:usize,ab:[(Usize1,Usize1);m],
        k:usize,xy:[(Usize1,Usize1);k],
        q:usize,pq:[(Usize1,Usize1);q],
    }
    let set = ab
        .iter()
        .map(|&(a, b)| (a, b))
        .collect::<HashSet<(usize, usize)>>();
    for (a, b) in pq {
        if set.contains(&(a, b)) || set.contains(&(b, a)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}

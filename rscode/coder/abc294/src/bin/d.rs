#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , q:usize,
    }
    let mut waitb: BTreeSet<usize> = BTreeSet::new();
    let mut wait: VecDeque<usize> = (0..n).collect();
    for _ in 0..q {
        input! {
            cas:usize,
        }
        if cas == 1 {
            let b: usize = wait.pop_front().unwrap();
            waitb.insert(b);
        } else if cas == 2 {
            input! {
                x:Usize1,
            }
            waitb.remove(&x);
        } else {
            let decoy = waitb.iter().next().unwrap();
            println!("{}", decoy + 1);
        }
    }
}

use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::collections::BinaryHeap;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut v: Vec<BinaryHeap<usize>> = vec![BinaryHeap::new(); n];
    let mut v2: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for _ in 0..q {
        input! {
            a:usize,
        }
        if a == 1 {
            input! {
                i:usize,j:Usize1,
            }
            v[j].push(i);
            v2[j].insert(i);
        } else if a == 2 {
            input! {
                i:Usize1,
            }
            let mut graph = v[i].iter().map(|x| *x).collect::<Vec<usize>>();
            graph.sort();
            println!("{}", graph.iter().join(" "));
        } else {
            input! {
                i:usize,
            }
            let mut set: BTreeSet<usize> = BTreeSet::new();
            for k in 0..n {
                if v2[k].contains(&i) {
                    set.insert(k);
                }
            }
            println!("{}", set.iter().map(|x| x + 1).join(" "));
        }
    }
}

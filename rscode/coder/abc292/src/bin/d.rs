#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize ,
        ab:[(Usize1 , Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in ab.iter() {
        uf.union(*a, *b);
    }
    let jd: Vec<usize> = uf.into_labeling();
    let mut result: Vec<usize> = vec![];
    for i in 0..m {
        if jd.iter().filter(|x| x == &&i).count() != 0 {
            result.push(jd.iter().filter(|x| x == &&i).count());
        }
    }
}

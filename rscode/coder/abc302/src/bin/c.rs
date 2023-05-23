#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,
        s:[Chars;n],
    }
    let mut jd = true;
    let mut cnt: usize = 0;
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                for k in 0..m {
                    if s[i][k] != s[j][k] {
                        if cnt == 1 {
                            cnt = 0;
                            jd = false;
                            break;
                        } else {
                            cnt += 1;
                        }
                    }
                }
            }
            if jd {
                uf.union(i, j);
            }
            jd = true;
            cnt = 0;
        }
    }
    let v = uf.into_labeling();
    let set = v.iter().map(|&x| x).collect::<HashSet<usize>>();
    if set.len() != 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}

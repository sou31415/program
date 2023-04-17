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
        a:[Chars;n],
        b:[Chars;n],
    }
    let mut c = a.clone();
    for k in 0..4{
        let mut jd = false;
        for i in 0..n{
            for j in 0..n{
                if c[i][j] == '1' && c[i][j] != '1' {
                    jd = true;
                    break;
                }
            }
            if jd{
                break;
            }
        }
        for i in 0..n{
            for j in 0..n{

        if k == 3{
            println!("Yes");
        }
    }
}

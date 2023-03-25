//submit passed
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n:usize,
        strlist:[String;n],
    }
    for i in 0..n {
        println!("{}", strlist[n - i - 1]);
    }
}

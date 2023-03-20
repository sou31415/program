use petgraph::{algo::is_cyclic_directed, prelude::DiGraphMap};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(String, String); n]
    }

    let s = s.iter().map(|(x, y)| (x.as_str(), y.as_str()));
    let g: DiGraphMap<_, ()> = s.collect();

    println!("{}", if is_cyclic_directed(&g) { "No" } else { "Yes" });
}

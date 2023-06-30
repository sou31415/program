#![allow(unused_imports)]
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader, Write};
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n:usize,
    }
    let mut l: usize = 1;
    let mut r: usize = n;
    while r - l > 1 {
        println!("? {}", (r + l) / 2);
        input! {
            from &mut source,
            m:usize,
        }
        if m == 0 {
            l = (r + l) / 2;
        } else {
            r = (r + l) / 2;
        }
    }
    println!("! {}", l);
}

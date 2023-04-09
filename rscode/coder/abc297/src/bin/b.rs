#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    let mut b: (usize, usize) = (0, 0);
    let mut bc: usize = 0;
    let mut k: (usize, usize) = (0, 0);
    let mut kc: usize = 0;
    let mut k2: usize = 0;
    for i in 0..s.len() {
        if s[i] == 'R' && kc == 0 {
            kc += 1;
            k.0 = i;
        } else if s[i] == 'R' && kc == 1 {
            kc += 1;
            k.1 = i;
        } else if s[i] == 'B' && bc == 0 {
            bc += 1;
            b.0 = i;
        } else if s[i] == 'B' && bc == 1 {
            bc += 1;
            b.1 = i;
        } else if s[i] == 'K' {
            k2 = i;
        }
    }
    if (b.1 - b.0) % 2 != 0 && k2 <= k.1 && k2 >= k.0 {
        println!("Yes");
        return;
    }
    println!("No");
}

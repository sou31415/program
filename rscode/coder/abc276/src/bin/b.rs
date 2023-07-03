use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
fn main() {
    input! {
      n:usize,m:usize,
      ab:[(Usize1,Usize1);m]
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        g[i].sort();
    }
    for i in 0..n {
        print!("{} ", g[i].len());
        println!("{}", g[i].iter().map(|x| *x + 1).join(" "));
    }
}

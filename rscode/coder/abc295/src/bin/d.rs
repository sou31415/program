#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:String,
    }
    let mut dp: Vec<usize> = vec![0; 1 << 10];
    let mut result: usize = 0;
    dp[0] += 1;
    let mut place: usize = 0;
    let list = to_int(s);
    for &i in &list {
        place ^= 1 << i;
        result += dp[place];
        dp[place] += 1;
    }
    println!("{}", result);
}

fn to_int(a: String) -> Vec<usize> {
    let v = a
        .chars()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    v
}

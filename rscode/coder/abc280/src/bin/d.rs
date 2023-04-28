#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
      k:u128,
    }
    if is_prime(k as usize) {
        println!("{}", k);
        return;
    }
    let mut a: Vec<u128> = vec![1, 1];
    loop {
        a.push(a.len() as u128 * a[a.len() - 1]);
        if a[a.len() - 1] % k == 0 {
            println!("{}", a.len() - 1);
            return;
        }
    }
}

pub fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in (2..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

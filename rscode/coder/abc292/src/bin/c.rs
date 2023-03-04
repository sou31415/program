#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    //let mut set = HashSet::new();
    let mut result = 0;
    let mut list: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];
    let mut jd: bool = false;
    for i in 1..=(n / 2) {
        list = gen_divisors(n - i);
        list2 = gen_divisors(i);
        if n - i != i {
            result += list.len() * list2.len();
        } else {
            jd = true;
        }
    }

    result *= 2;
    if jd {
        result += list.len() * list2.len();
    }
    println!("{}", result);
}

fn gen_divisors(n: usize) -> Vec<usize> {
    let mut res = vec![];

    for i in (1..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            if i * i == n {
                res.push(i);
            } else {
                res.push(i);
                res.push(n / i);
            }
        }
    }
    res.sort();

    res
}

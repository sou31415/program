#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
      n:usize,ab:[(usize,usize);n],
    }
    let mut imos1: Vec<usize> = vec![0];
    let mut imos2: Vec<usize> = vec![0];

    for (a, b) in ab {
        if a == 1 {
            let nlen: usize = imos1.len() - 1;
            imos1.push(imos1[nlen] + b);
            imos2.push(imos2[imos2.len() - 1]);
        } else {
            let nlen: usize = imos2.len() - 1;
            imos2.push(imos2[nlen] + b);
            imos1.push(imos1[imos1.len() - 1]);
        }
    }
    input! {
      q:usize,
    }
    for _ in 0..q {
        input! {
          l:usize , r:usize,
        }
        println!("{} {}", imos1[r] - imos1[l - 1], imos2[r] - imos2[l - 1]);
    }
}

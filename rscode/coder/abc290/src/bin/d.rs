#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        input! {
            n:usize , d:usize , k:usize,
        }
        let mut x: usize = 0;
        let mut dp = vec![false; n]; //どこを塗ったか
        dp[0] = true; //初期条件
                      //let mut indexed: usize = 0;
        for _ in 0..(k - 1) {
            x = (x + d) % n;
            while dp[x] {
                x = (x + 1) % n;
            }
            dp[x] = true;
            //indexed = x;
        }
        println!("{}", x);
    }
}

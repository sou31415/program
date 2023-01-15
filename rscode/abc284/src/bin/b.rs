//submit passed
use proconio::input;

fn main() {
    input! {
        n:u64,
    }
    for _ in 0..n {
        let mut cnt = 0;
        input! {
            a:u64,
            inlist:[u64;a],
        }
        for i in inlist {
            if i % 2 == 1 {
                cnt += 1;
            }
        }
        println!("{}", cnt);
    }
}

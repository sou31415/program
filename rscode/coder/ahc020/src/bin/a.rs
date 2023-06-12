use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,k:usize,
        xyz:[(isize,isize);n],
        uvw:[(Usize1,Usize1,usize);m],
        abcde:[(isize,isize);k],
    }
    for i in 0..n {
        let mut s = rand::thread_rng();
        print!("{} ", s.gen_range(0, 5000));
    }
    println!("");
    for _ in 0..m {
        let mut s = rand::thread_rng();
        if s.gen_range(0, 90000006) % 5 == 0 {
            print!("0 ");
        } else {
            print!("1 ");
        }
    }
}

use proconio::{fastout, input, marker::Usize1};
use rand::Rng;
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,k:usize,
        xy:[(isize,isize);n],
        uv:[(Usize1,Usize1,usize);m],
        ab:[(isize,isize);k],
    }
}

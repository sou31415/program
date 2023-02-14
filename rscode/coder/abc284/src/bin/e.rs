use proconio::{marker::Usize1 ,input};

fn main(){
    input!{
        n:usize , m:usize,
        ab :[(Usize1 , Usize1);m],
    }
    let mut edge = vec![vec![]];
    for (a , b) in ab{
        edge[a].push(b);
        edge[b].pusb(a);
    }
}

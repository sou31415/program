use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize , m:usize,
        ab :[(Usize1 , Usize1);m],
    }
    let mut node: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        node[a].push(b);
        node[b].push(a);
    }
    let mut jg:Vec<bool>
    let mut seen: Vec<usize> = node[0].clone();
    let mut result: usize = 1;
    while seen.len() != 0{
        let edge:usize = seen.pop.unwrap();
        seen.append(&mut node[edge]);

}

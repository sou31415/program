use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1 , Usize1);n],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![];1000000001];
    for (a ,b) in ab{
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans:usize = dfs(mut g);
}

fn dfs(g:mut Vec<Vec<usize>>) -> usize{
    let mut  v:Vec<usize> = vec![];
    while let Some(x) = g[0].pop(){
        v.push(x);
    }
    while let Some(x) = v.pop(){


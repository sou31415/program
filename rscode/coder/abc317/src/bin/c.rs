use proconio::{input,marker::Usize1};
use std::cmp::max;
fn main(){
    input!{
        n:usize,m:usize,
        abc:[(Usize1,Usize1,usize);m]
    }
    let mut g:Vec<Vec<(usize,usize)>> = vec![vec![];n];
    for (a,b,c) in abc{
        g[a].push((b,c));
        g[b].push((a,c));
    }
    let mut result:usize = 0;
    for i in 0..n{
        result = max(result,dfs(n,i,g.clone()));
    }
    println!("{}",result);
}

fn dfs(n:usize,s:usize,mut g:Vec<Vec<(usize,usize)>>) -> usize{
    let mut seen = vec![false;n];
    let mut stack = vec![];
    while let Some((x,a)) = g[s].pop(){
        stack.push((x,a));
}

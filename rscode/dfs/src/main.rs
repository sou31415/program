use proconio::input;

fn main(){
    input! {
        v:usize,
        e:usize,
        s:usize,
        t:usize,
    }
    for _ in 0..e {
        input! {
            a:usize,
            b:usize,
        }
        let mut g : Vec<Vec<usize>> = Vec::new();
        g[a].push(b);
}

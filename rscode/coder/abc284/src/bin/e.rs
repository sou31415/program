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
    let mut result: usize = 1;
    if node[0].is_empty() {
        println!("1");
        return;
    }
    let mut hashes: usize = 0;
    let mut stack: Vec<(usize, usize)> = vec![];
    for i in 0..node[0].len() {
        let x = node[0][i];
        if x ^ hashes == x - hashes && x != 0 {
            result += 1;
            stack.push((x, x | hashes));
        }
    }
    while let Some((x, z)) = stack.pop() {
        for i in 0..node[x].len() {
            let y = node[x][i];
            if result > 1000000 {
                println!("1000000");
                return;
            }
            if z ^ y == (z as isize - y as isize).abs() as usize && y != 0 {
                result += 1;
                stack.push((y, z | y));
            }
        }
    }
    println!("{}", result);
}

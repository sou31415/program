use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,k:usize,
        xy:[(isize,isize);n],
        uvw:[(Usize1,Usize1,usize);m],
        ab:[(isize,isize);k],
    }
    let mut pq: BinaryHeap<(isize, usize, usize, usize)> = BinaryHeap::new();
    for i in 0..m {
        let (a, b, c) = uvw[i];
        pq.push((0 - c as isize, a, b, i));
    }
    let mut d: Vec<Vec<isize>> = vec![vec![0; n]; k];
    for i in 0..k {
        for j in 0..n {
            d[i][j] = d2(ab[i].0, ab[i].1, xy[j].0, xy[j].1);
        }
    }
    let mut trial: Vec<Vec<isize>> = vec![vec![0; n]; n];
    let mut adj: Vec<(usize, isize)> = vec![(0, 0); k];
    for i in 0..k {
        let mut pos = 0;
        let mut pow = std::isize::MAX;
        for j in 0..n {
            if d[i][j] < pow {
                pos = j;
                pow = d[i][j];
            }
        }
        adj[i] = (pos, pow);
    }
    let mut need: Vec<usize> = vec![0; n];
    for i in 0..k {
        trial[n - 1][adj[i].0] = max(trial[n - 1][adj[i].0], adj[i].1);
        need[adj[i].0] += 1;
    }
    let mut best_when = n - 1;
    let mut best_cost: isize = 0;
    for i in 0..n {
        best_cost += trial[n - 1][i];
    }
    let mut freq: Vec<(usize, usize)> = vec![(0, 0); n];
    for i in 0..n {
        freq[i] = (need[i], i);
    }
    freq.sort();
    freq.reverse();
    let mut avai: HashSet<usize> = HashSet::new();

    for n in 0..n - 1 {
        let mut tmp: Vec<isize> = vec![0; n];
        let mut can = true;
        avai.insert(freq[n].1);
        for i in 0..k {
            let mut pos = 0;
            let mut pow = std::isize::MAX;
            for j in 0..n {
                if d[i][j] < pow && avai.contains(&j) {
                    pos = j;
                    pow = d[i][j];
                }
            }

            adj[i] = (pos, pow);
        }
        for i in 0..k {
            trial[n][adj[i].0] = max(trial[n][adj[i].0], adj[i].1);
        }
        let mut tmp_cost: isize = 0;
        for i in 0..n {
            tmp_cost += tmp[i];
            if tmp[i] > 5000 * 5000 {
                can = false;
            }
        }
        if can && tmp_cost < best_cost {
            best_when = n;
            best_cost = tmp_cost;
        }
    }

    let mut p: Vec<isize> = vec![0; n];
    for i in 0..n {
        p[i] = ((trial[best_when][i] as f64).sqrt().ceil()) as isize;
    }
    let mut sw: Vec<isize> = vec![0; m];
    let mut mst: Vec<HashSet<Vec<isize>>> = vec![HashSet::new(); n];
    let mut uf = UnionFind::new(n);
    let mut cnt: usize = (0..n).filter(|&x| uf.equiv(0, x)).count();
    while cnt < n {
        if let Some((v0, v1, v2, v3)) = pq.pop() {
            if !uf.equiv(v1, v2) {
                uf.union(v1, v2);
                sw[v3] = 1;
                mst[v1].insert(vec![v1 as isize, v2 as isize, v3 as isize]);
                mst[v2].insert(vec![v2 as isize, v1 as isize, v3 as isize]);
            }
            cnt = (0..n).filter(|&x| uf.equiv(0, x)).count();
        }
    }
    let mut rmve: Vec<Vec<isize>> = Vec::new();
    for i in 0..n {
        if mst[i].len() == 1 {
            let v = mst[i].iter().next().unwrap();
            rmve.push(v.to_vec());
        }
    }
    while !rmve.is_empty() {
        let v = rmve.pop().unwrap();
        if p[v[0] as usize] == 0 {
            sw[v[2] as usize] = 0;
            mst[v[1] as usize].remove(&vec![v[1], v[0], v[2]]);
        }
        if mst[v[1] as usize].len() == 1 {
            let v = mst[v[1] as usize].iter().next().unwrap();
            rmve.push(v.to_vec());
        }
    }
    println!("{}", p.iter().join(" "));
    println!("{}", sw.iter().join(" "));
}

fn d2(a: isize, b: isize, c: isize, d: isize) -> isize {
    let k = ((a - c).abs() * (a - c).abs() + (d - b).abs() * (d - b).abs()) as isize;
    k
}

use petgraph::{
    algo::{toposort, DfsSpace},
    graph::DiGraph,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        xy:[(Usize1 , Usize1);n],
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let g: DiGraph<usize, usize> = DiGraph::from_edges(&xy);
    let mut space = DfsSpace::new(&g);
    let result = toposort(&g, Some(&mut space));
    let nodes: Vec<usize> = result.unwrap().iter().map(|x| x.index()).collect();
}

#[warn(unused_imports)]
use example::conn_comps_undirected_graph;
use example::topsort_dfs_dag;
use example::kruskal;
use std::collections::{HashMap, HashSet};
mod example;

type Graph = HashMap<Vert, HashSet<Vert>>;
type WeightedGraph = HashMap<Vert, HashSet<(Vert, u32)>>;

fn add_edge(graph : &mut WeightedGraph, wt: u32, v1: Vert, v2: Vert) {
    graph.entry(v1).and_modify(|hs| {
        hs.insert((v2, wt));
    });

    graph.entry(v2).and_modify(|hs| {
        hs.insert((v1, wt));
    });
}

#[derive(Eq, Hash, PartialOrd, Ord, PartialEq, Debug, Copy, Clone)]
pub struct Vert {
    pub val: i32,
}

impl Vert {
    pub fn new(val: i32) -> Self {
        Vert { val }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn test_conn_comps_undirected_graph() {
    let p: usize = 7;

    let n: i32 = 100;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: Graph = HashMap::with_capacity(un);
    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    for i in 0..un {
        for j in 0..un {
            if i % p == j % p && i != j {
                let vi = Vert { val: i as i32 };
                let vj = Vert { val: j as i32 };

                graph.entry(vi).and_modify(|hs| {
                    hs.insert(vj);
                });
                graph.entry(vj).and_modify(|hs| {
                    hs.insert(vi);
                });
            }
        }
    }

    conn_comps_undirected_graph::doit(&verts, &graph);

    //    assert_eq!(3, 4)
}

fn test_topsort_dfs_dag() {
    let n: i32 = 6;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: Graph = HashMap::with_capacity(un);
    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    graph.entry(verts[5]).and_modify(|hs| {
        hs.insert(verts[2]);
    });
    graph.entry(verts[5]).and_modify(|hs| {
        hs.insert(verts[0]);
    });
    graph.entry(verts[4]).and_modify(|hs| {
        hs.insert(verts[1]);
    });
    graph.entry(verts[4]).and_modify(|hs| {
        hs.insert(verts[0]);
    });
    graph.entry(verts[3]).and_modify(|hs| {
        hs.insert(verts[1]);
    });
    graph.entry(verts[2]).and_modify(|hs| {
        hs.insert(verts[3]);
    });

    let mut stack: Vec<Vert> = Vec::new();
    topsort_dfs_dag::doit(&verts, &graph, &mut stack);

    println!("here is topsort stack: ");
    for vv in stack.iter().rev() {
        println!("{:?}", vv);
    }
}

fn test_kruskal() {
    let n: i32 = 15;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: WeightedGraph = HashMap::with_capacity(un);

    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    add_edge(&mut graph,0, verts[1], verts[4]);
    add_edge(&mut graph,0, verts[7], verts[8]);
    add_edge(&mut graph,1, verts[2], verts[8]);
    add_edge(&mut graph,1, verts[7], verts[11]);
    add_edge(&mut graph,2, verts[3], verts[7]);
    add_edge(&mut graph,2, verts[8], verts[2]);
    add_edge(&mut graph,2, verts[5], verts[4]);
    add_edge(&mut graph,3, verts[4], verts[9]);
    add_edge(&mut graph,3, verts[5], verts[14]);
    add_edge(&mut graph,4, verts[5], verts[10]);
    add_edge(&mut graph,5, verts[6], verts[2]);
    add_edge(&mut graph,6, verts[7], verts[1]);
    add_edge(&mut graph,6, verts[8], verts[6]);
    add_edge(&mut graph,7, verts[8], verts[7]);

    let wt = kruskal::doit(&verts, &graph);

    println!("yo, the min spanning tree weight is: {}", wt);
}

fn main() {
    println!("testing conn comps-----------");
    test_conn_comps_undirected_graph();

    println!("testing topsort------------");
    test_topsort_dfs_dag();

    println!("testing kruskal--------------");
    test_kruskal();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

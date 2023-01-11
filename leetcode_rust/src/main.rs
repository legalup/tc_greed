#[warn(unused_imports)]
use example::conn_comps_undirected_graph;
use example::topsort_dfs_dag;
use std::collections::{HashMap, HashSet};
mod example;

type Graph = HashMap<Vert, HashSet<Vert>>;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
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

fn main() {
    println!("testing conn comps-----------");
    test_conn_comps_undirected_graph();

    println!("testing topsort------------");
    test_topsort_dfs_dag();
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

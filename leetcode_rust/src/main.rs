#[warn(unused_imports)]
use example::conn_comps_undirected_graph;
use example::kruskal;
use example::partition_generator;
use example::shortest_path_iterative_dag;
use example::shortest_path_recursive_dag;
use example::sparse_vector::SparseVector;
use example::topsort_dfs_dag;
use example::union_find_detect_cycle;
use example::sparse_vector;
use std::collections::{HashMap, HashSet};
mod example;

type Graph = HashMap<Vert, HashSet<Vert>>;
type WeightedGraph = HashMap<Vert, HashSet<(Vert, i32)>>;

fn add_weighted_edge(graph: &mut WeightedGraph, wt: i32, v1: Vert, v2: Vert) {
    graph.entry(v1).and_modify(|hs| {
        hs.insert((v2, wt));
    });

    graph.entry(v2).and_modify(|hs| {
        hs.insert((v1, wt));
    });
}

fn add_edge(graph: &mut Graph, v1: Vert, v2: Vert) {
    graph.entry(v1).and_modify(|hs| {
        hs.insert(v2);
    });

    graph.entry(v2).and_modify(|hs| {
        hs.insert(v1);
    });
}

fn add_directed_weighted_edge(graph: &mut WeightedGraph, wt: i32, v1: Vert, v2: Vert) {
    graph.entry(v1).and_modify(|hs| {
        hs.insert((v2, wt));
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
    let n: i32 = 4;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: WeightedGraph = HashMap::with_capacity(un);

    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    add_weighted_edge(&mut graph, 5, verts[0], verts[1]);
    add_weighted_edge(&mut graph, 10, verts[2], verts[1]);
    add_weighted_edge(&mut graph, 15, verts[2], verts[3]);
    add_weighted_edge(&mut graph, 20, verts[0], verts[3]);
    add_weighted_edge(&mut graph, 1, verts[0], verts[2]);
    add_weighted_edge(&mut graph, 2, verts[3], verts[1]);

    let wt = kruskal::doit(&verts, &graph);

    println!("yo, the min spanning tree weight is: {}", wt);
}

fn test_union_find_detect_cycle() {
    let n: i32 = 4;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: Graph = HashMap::with_capacity(un);

    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    add_edge(&mut graph, verts[0], verts[1]);
    add_edge(&mut graph, verts[2], verts[1]);
    add_edge(&mut graph, verts[2], verts[3]);
    add_edge(&mut graph, verts[0], verts[3]);

    let hascycle = union_find_detect_cycle::doit(&verts, &graph);

    if hascycle {
        println!("it does have a cycle")
    } else {
        print!("it does NOT have a cycle.");
    }
}

fn test_shortest_path_iterative_dag() {
    let n: i32 = 6;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: WeightedGraph = HashMap::with_capacity(un);

    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    add_directed_weighted_edge(&mut graph, 5, verts[0], verts[1]);
    add_directed_weighted_edge(&mut graph, 3, verts[0], verts[2]);
    add_directed_weighted_edge(&mut graph, 6, verts[1], verts[3]);
    add_directed_weighted_edge(&mut graph, 2, verts[1], verts[2]);
    add_directed_weighted_edge(&mut graph, 4, verts[2], verts[4]);
    add_directed_weighted_edge(&mut graph, 2, verts[2], verts[5]);
    add_directed_weighted_edge(&mut graph, 7, verts[2], verts[3]);
    add_directed_weighted_edge(&mut graph, -1, verts[3], verts[4]);
    add_directed_weighted_edge(&mut graph, -2, verts[4], verts[5]);

    let poopy = shortest_path_iterative_dag::doit(&verts, &graph, verts[1]);

    println!("here is poopy {:?}", poopy);
}

fn test_shortest_path_recursive_dag() {
    let n: i32 = 6;
    let un = n as usize;
    let mut verts: Vec<Vert> = Vec::with_capacity(un);
    let mut graph: WeightedGraph = HashMap::with_capacity(un);

    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
        graph.insert(vv, HashSet::with_capacity(un));
    }

    add_directed_weighted_edge(&mut graph, 5, verts[0], verts[1]);
    add_directed_weighted_edge(&mut graph, 3, verts[0], verts[2]);
    add_directed_weighted_edge(&mut graph, 6, verts[1], verts[3]);
    add_directed_weighted_edge(&mut graph, 2, verts[1], verts[2]);
    add_directed_weighted_edge(&mut graph, 4, verts[2], verts[4]);
    add_directed_weighted_edge(&mut graph, 2, verts[2], verts[5]);
    add_directed_weighted_edge(&mut graph, 7, verts[2], verts[3]);
    add_directed_weighted_edge(&mut graph, -1, verts[3], verts[4]);
    add_directed_weighted_edge(&mut graph, -2, verts[4], verts[5]);

    for i in 0..un{
        let poopy = shortest_path_recursive_dag::doit(&verts, &graph, verts[i]);
        println!("short path from 1 to {} is {}", i, poopy);
    }
    

}

fn test_sparse_vector(){

    let testvec = vec![1,2,3,4,5,6,7,8,9];

    let mut sv1= SparseVector::from_vec(testvec);
    let mut sv2 = SparseVector::new();  

    sv2.keys.insert(2);
    sv2.ns.insert(2,100);

    println!("here is the innerproduct {}", sv1.dot_product(&mut sv2));

}   
fn main() {
    println!("testing conn comps-----------");
    test_conn_comps_undirected_graph();

    println!("testing topsort------------");
    test_topsort_dfs_dag();

    println!("testing kruskal--------------");
    test_kruskal();

    println!("testing cycle detect");
    test_union_find_detect_cycle();

    println!("testing partition generator");
    partition_generator::doit(3, 7);

    println!("testing shortest path iterative");
    test_shortest_path_iterative_dag();

    println!("testing shortest path recursive");
    test_shortest_path_recursive_dag();

    println!("testing sparse vector");
    test_sparse_vector();
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

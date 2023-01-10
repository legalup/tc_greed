#[warn(unused_imports)]
use example::conn_comps_undirected_graph::{doit};
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
    for i in 0..n {
        let vv = Vert { val: i };
        verts.push(vv);
    }

    let mut graph : Graph = HashMap::with_capacity(un);

    for i in 0..un {
        for j in 0..un {
            if i % p == j % p && i != j {
                let vi = Vert { val: i as i32 };
                let vj = Vert { val: j as i32 };

                if graph.contains_key(&vi) {
                    graph.get_mut(&vi).expect("go fuck yerself").insert(vj);
                } else {
                    let mut verty: HashSet<Vert> = HashSet::with_capacity(un);
                    verty.insert(vj);
                    graph.insert(vi, verty);
                }

                if graph.contains_key(&vj) {
                    graph.get_mut(&vj).expect("go fuck yerself").insert(vi);
                } else {
                    let mut verty: HashSet<Vert> = HashSet::with_capacity(un);
                    verty.insert(vi);
                    graph.insert(vj, verty);
                }
            }
        }
    }

    doit(&verts, &graph);

    //    assert_eq!(3, 4)
}

fn main() {
    test_conn_comps_undirected_graph();
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

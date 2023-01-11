#![allow(unused)]

use crate::{Graph, Vert};
use std::collections::HashSet;

pub fn topsort<'a>(
    v: &'a Vert,
    graph: &'a Graph,
    visited: &mut HashSet<&'a Vert>,
    stack: &mut Vec<Vert>,
) {
    visited.insert(v);

    for nbr in graph[v].iter() {
        if !visited.contains(nbr) {
            topsort(nbr, graph, visited, stack);
        }
    }
    stack.push(*v);
}

pub fn doit(verts: &[Vert], graph: &Graph, stack: &mut Vec<Vert>) {
    let mut visited: HashSet<&Vert> = HashSet::new();

    for v in verts.iter() {
        if !visited.contains(v) {
            topsort(v, graph, &mut visited, stack);
        }
    }
}

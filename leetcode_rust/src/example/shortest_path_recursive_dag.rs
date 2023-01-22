#![allow(unused)]

use crate::{Vert, WeightedGraph};
use std::cmp;
use std::collections::{HashMap};

// this is recursive from the end. so, we assume that the distance to the end
// has already been set to zero. so this function return the shortest path from
// start to the vert where dist has been set to zero
fn shortest_path(
    verts: &[Vert],
    graph: &WeightedGraph,
    start: Vert,
    dist: &mut HashMap<Vert, i32>,
) -> i32 {
    if dist.contains_key(&start) {
        return dist[&start];
    }

    dist.insert(start, 1000000);

    for v in graph[&start].iter() {
        let dd = cmp::min(dist[&start], v.1 + shortest_path(verts, graph, v.0, dist));
        dist.insert(start, dd);
    }

    dist[&start]
}

//this returns the distance from vertex 1 to the vert you are entering here
// recurses backward
pub fn doit(verts: &[Vert], graph: &WeightedGraph, end: Vert) -> i32 {
    let mut dist: HashMap<Vert, i32> = HashMap::new();

    dist.insert(end, 0);

    shortest_path(verts, graph, verts[1], &mut dist)

}

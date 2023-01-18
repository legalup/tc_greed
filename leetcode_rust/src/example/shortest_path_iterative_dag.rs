#![allow(unused)]

use crate::{Vert, WeightedGraph};
use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn topsort<'a>(
    v: &'a Vert,
    graph: &'a WeightedGraph,
    visited: &mut HashSet<&'a Vert>,
    stack: &mut Vec<Vert>,
) {
    visited.insert(v);

    for nbr in graph[v].iter() {
        if !visited.contains(&nbr.0) {
            topsort(&nbr.0, graph, visited, stack);
        }
    }
    stack.push(*v);
}

pub fn doit(verts: &[Vert], graph: &WeightedGraph, start: Vert) -> HashMap<Vert, i32> {
    let mut visited: HashSet<&Vert> = HashSet::new();

    let mut stack: Vec<Vert> = Vec::new();

    for v in verts.iter() {
        if !visited.contains(v) {
            topsort(v, graph, &mut visited, &mut stack);
        }
    }

    let mut dist: HashMap<Vert, i32> = HashMap::new();

    dist.insert(start, 0);

    while !stack.is_empty() {
        let z = stack.pop().unwrap();

        if dist.contains_key(&z) {
            for v in graph[&z].iter() {
                if dist.contains_key(&v.0) {
                    let dd = cmp::min(dist[&v.0], dist[&z] + v.1);
                    dist.insert(v.0, dd);
                } else {
                    dist.insert(v.0, dist[&z] + v.1);
                }
            }
        }
    }
    return dist;
}

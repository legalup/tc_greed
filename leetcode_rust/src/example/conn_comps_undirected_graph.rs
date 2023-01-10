#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

use crate::Graph;
use crate::Vert;

//pub static mut Verts: Vec<Vert> = Vec::with_capacity(n);

//static mut graph: HashMap<Vert, Vec<Vert>> = HashMap::with_capacity(n);

pub fn dfs<'a>(
    v: &'a Vert,
    graph: &'a Graph,
    visited: &mut HashSet<&'a Vert>,
    comp: &mut HashSet<&'a Vert>,
) {
    visited.insert(v);
    comp.insert(v);
    //println!("dfs. size of used is {}", used.len());
    //println!(" in dfs. here is what is in v: {:?}", v);

    for nbr in graph[v].iter() {
        if !visited.contains(nbr) {
            dfs(nbr, graph, visited, comp);
        }
    }
}

pub fn doit(verts: &[Vert], graph: &Graph) {
    let mut visited: HashSet<&Vert> = HashSet::new();
    let mut comp: HashSet<&Vert> = HashSet::new();

    visited.clear();

    
    for v in verts.iter() {
        
        comp.clear();

        if !visited.contains(&v) {
            dfs(v, graph, &mut visited, &mut comp);
            println!(" component: {:?}", comp);
        }
    }
}

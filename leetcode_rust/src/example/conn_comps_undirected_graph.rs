#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct Vert {
    pub val: i32,
}

impl Vert {
    pub fn new(val: i32) -> Self {
        Vert { val }
    }
}

//pub static mut Verts: Vec<Vert> = Vec::with_capacity(n);

//static mut graph: HashMap<Vert, Vec<Vert>> = HashMap::with_capacity(n);

pub fn dfs<'a>(
    v: &'a Vert,
    graph: &'a HashMap<Vert, HashSet<Vert>>,
    used: &mut HashSet<&'a Vert>,
    comp: &mut HashSet<&'a Vert>,
) {
    used.insert(v);
    comp.insert(v);
    //println!("dfs. size of used is {}", used.len());
    //println!(" in dfs. here is what is in v: {:?}", v);

    for nbr in graph[v].iter() {
        if !used.contains(nbr) {
            dfs(nbr, graph, used, comp);
        }
    }
}

pub fn doit(verts: &[Vert], graph: &HashMap<Vert, HashSet<Vert>>) {
    let mut used: HashSet<&Vert> = HashSet::new();

    let mut comp: HashSet<&Vert> = HashSet::new();

    used.clear();

    //println!("doit. size of graph is {}", graph.len());
    for v in verts.iter() {
        //println!("doit. here is v:{:?} in verts", v);
        comp.clear();
        let mut nuused = used.clone();
        if !used.contains(&v) {
            dfs(v, graph, &mut nuused, &mut comp);
            println!(" component: {:?}", comp);
        }
        //now update used with what we learned
        for vv in nuused {
            used.insert(vv);
        }
    }
}

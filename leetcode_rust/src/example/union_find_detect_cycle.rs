#![allow(unused)]

use crate::{Vert, Graph};
use std::collections::{HashMap, HashSet};

/********************************
 * Given an undirected connected and weighted grasph, find the minimum
 * spanning tree. this uses the union find algo. this also works
 * if vert type is set to pair<int,int>. except for main, of course.
 *
 * Huzzah!
 ****************************************/

//this feature has path compression
pub fn find_set(mut v: Vert, parent: &mut HashMap<Vert, Vert>) -> Vert {
    if v == parent[&v] {
        return v;
    }

    let p = find_set(parent[&v], parent);
    parent.insert(v, p);

    p
}

//this particular union set choose the parent that is the
//smallest in dictionary ordering. theres many other ways to go here
//especially to optimize for size & rank.
fn union_sets(
    mut a: Vert,
    mut b: Vert,
    parent: &mut HashMap<Vert, Vert>
) {
    //is mut a really necessary
    a = find_set(a, parent);
    b = find_set(b, parent);

    if a <= b {
        parent.insert(a,b);
    }
    else {
        parent.insert (b,a);
    }

    
}

//////////////////////////////////////////////
// disjoint union. great for finding if a cycle exists in a graph
//////////////////////////////////////////////////////////////

pub fn doit(verts: &[Vert], graph: &Graph) -> bool {
    let mut visited: HashSet<&Vert> = HashSet::new();

    //lets make the weights
    //this guarantees i only get one of each
    let mut edges = HashSet::new();
    for (key, val) in graph.iter() {
        for vv in val.iter() {
            if *key <= *vv
            {
                let z = (*key,  *vv);
                edges.insert(z);
            } else {
                edges.insert((*vv, *key));
            }
        }
    }

    // look at me converting a hashset to a vec. yay 4 iters
    // doing this for the sort
    // and here is a lovely turbofish...
    let mut edge_vec = edges.into_iter().collect::<Vec<_>>();

    //needed data structures
    let mut parent: HashMap<Vert, Vert> = HashMap::new();

    for vv in verts.iter() {
        parent.insert(*vv, *vv);
    }

    for z in edge_vec.iter() {
        let set_u = find_set(z.0, &mut parent);
        let set_v = find_set(z.1, &mut parent);

        if set_u == set_v {
            return true;    
        }
        
        union_sets(z.1, z.0, &mut parent);
    }

    return false;
}

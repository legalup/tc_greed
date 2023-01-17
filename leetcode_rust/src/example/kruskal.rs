#![allow(unused)]

use crate::{Vert, WeightedGraph};
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
    parent: &mut HashMap<Vert, Vert>,
    rnk: &mut HashMap<Vert, u32>,
) {
    //is mut a really necessary
    a = find_set(a, parent);
    b = find_set(b, parent);

    if a != b {
        if rnk[&a] < rnk[&b] {
            std::mem::swap(&mut a, &mut b);
        }
        //vert with highest rank is new parent
        parent.insert(b, a);
        if rnk[&a] == rnk[&b] {
            rnk.entry(a).and_modify(|cnt| {
                *cnt += 1;
            });
        }
        // for now, will use rank.
        /*
        if rnk[&a] < rnk[&b] {
            parent.insert(b, a);
        } else {
            parent.insert(a, b);
        }

        if rnk[&a] == rnk[&b] {
            if a <= b {
                rnk.entry(b).and_modify(|cnt| {
                    *cnt += 1;
                });
            } else {
                rnk.entry(a).and_modify(|cnt| {
                    *cnt += 1;
                });
            }
        }
        */
    }

    // can do other things. can look at some other natural ordering, depending
    // on problem
}

//////////////////////////////////////////////
// disjoint union. great for finding cc of an image and min spanning sets
//////////////////////////////////////////////////////////////

pub fn doit(verts: &[Vert], graph: &WeightedGraph) -> u32 {
    let mut visited: HashSet<&Vert> = HashSet::new();

    //lets make the weights
    //this guarantees i only get one of each
    let mut weights = HashSet::new();
    for (key, val) in graph.iter() {
        for vv in val.iter() {
            if *key <= vv.0 {
                let z = (vv.1, *key, vv.0);
                weights.insert(z);
            } else {
                weights.insert((vv.1, vv.0, *key));
            }
        }
    }

    // look at me converting a hashset to a vec. yay 4 iters
    // doing this for the sort
    // and here is a lovely turbofish...
    let mut weight_vec = weights.into_iter().collect::<Vec<_>>();
    weight_vec.sort();

    //needed data structures
    let mut parent: HashMap<Vert, Vert> = HashMap::new();
    let mut rnk: HashMap<Vert, u32> = HashMap::new();

    for vv in verts.iter() {
        parent.insert(*vv, *vv);
        rnk.insert(*vv, 0);
    }

    let mut mst_wt = 0;

    for z in weight_vec.iter() {
        let set_u = find_set(z.1, &mut parent);
        let set_v = find_set(z.2, &mut parent);

        if set_u != set_v {
            mst_wt += z.0;
            union_sets(z.1, z.2, &mut parent, &mut rnk);
        }
    }

    return mst_wt;
}

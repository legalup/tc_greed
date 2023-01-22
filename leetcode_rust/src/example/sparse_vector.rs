#![allow(unused)]

use crate::{Graph, Vert};
use std::collections::{HashMap, HashSet};

/********************************
class that represents sparse vector
****************************************/
#[derive(Default, Debug)]
pub struct SparseVector{
    pub ns : HashMap<i32,i32>,
    pub keys : HashSet<i32>,
}

impl SparseVector{

    pub fn new() -> Self{

        Self{
        ns : HashMap::new(),
        keys : HashSet::new(),
        }
    }

    pub fn from_vec(vec : Vec<i32>) -> Self{

        let mut ns = HashMap::new();
        let mut keys = HashSet::new();

        for (k, vv) in vec.into_iter().enumerate(){

            keys.insert(k as i32);
            ns.insert(k as i32,vv);
        }

        Self{
            ns : ns,
            keys : keys
        }
    }

    pub fn dot_product(self, other : &mut SparseVector)->i32{
        let mut sum = 0;
        for idx in self.keys.intersection(&mut other.keys){
            sum += self.ns[&idx]*other.ns[&idx];
        }
        sum
    }

}



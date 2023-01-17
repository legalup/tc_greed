#![allow(unused)]

use crate::{Graph, Vert};
use std::collections::{HashMap, HashSet};

/********************************
class that generates all ways of putting k things into n buckets
 ****************************************/

struct partition_generator_1 {
    k: i32,
    N: i32,
    state: Vec<i32>,
    is_done: bool,
}

impl partition_generator_1 {
    fn new(numobjs: i32, numbuckets: i32) -> Self {
        let mut ret = partition_generator_1 {
            k: numbuckets - 1,
            N: numbuckets + numobjs,
            state: Vec::new(),
            is_done: false,
        };

        for i in (1..(ret.k + 2)).rev() {
            ret.state.push(ret.N - i);
        }

        ret
    }

    fn next(&mut self) -> Vec<i32> {
        let mut ret = Vec::new();

        if self.is_done {
            return ret;
        }

        self.is_done = true;

        if self.state[0] > 0 {
            self.state[0] -= 1;
            self.is_done = false;
        } else {
            for i in 1..self.k as usize {
                if self.state[i] - self.state[i - 1] > 1 {
                    self.is_done = false;
                    self.state[i] -= 1;
                    for j in 1..(i + 1) {
                        self.state[i - j] = self.state[i] - j as i32;
                    }
                    break;
                }
            }
        }
        ret.push(self.state[0]);
        for i in 1..self.k as usize {
            ret.push(self.state[i] - self.state[i - 1] - 1);
        }

        ret.push(self.N - self.state[self.k as usize - 1] - 2);

        ret
    }
}

//class that generates all ways of putting k or less things into n buckets
struct partition_generator_2 { 
    numbuckets: i32,
    numobjs: i32,
    curr_numobjs: i32,
    pg1: Box<partition_generator_1>,
    is_done: bool,
}

impl partition_generator_2 {
    fn new(numobjs: i32, buckets: i32) -> Self {
        let mut ret = partition_generator_2 {
            numbuckets: buckets,
            numobjs: numobjs,
            curr_numobjs: numobjs,
            pg1: Box::new(partition_generator_1::new(numobjs, buckets)),
            is_done: false,
        };
        ret
    }

    fn next(&mut self) -> Vec<i32> {
        let ret = self.pg1.next();
        if !ret.is_empty() {
            return ret;
        }

        self.curr_numobjs -= 1;

        if self.curr_numobjs > 0 {
            self.pg1 = Box::new(partition_generator_1::new(self.curr_numobjs, self.numbuckets));
            return self.pg1.next();
        } else {
            self.is_done = true;
            return std::iter::repeat(0)
                .take(self.numbuckets as usize)
                .collect::<Vec<_>>();
        }

        ret
    }
}

pub fn doit(numobjs: i32, numbuckets: i32) {
    let mut pg1= partition_generator_2::new(numobjs, numbuckets);
    

    while !pg1.is_done {
        println!("{:?}", pg1.next());
    }
}

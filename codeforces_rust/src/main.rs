use std::env;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, prelude::*, stdin, stdout, BufReader, BufWriter, Result, Write};
use std::num::NonZeroUsize;
use std::ops::{Bound, RangeBounds};
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::{DefaultHasher, RandomState},
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
    },
};
const INF: i64 = 0x3f3f3f3f;
const MOD: usize = 1_000_000_007;
static USE_FILE_INPUT : bool=true;
// love this line:
// let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

pub fn doit( nums : &mut Vec::<(i32,i32)>) {
    println!("here is nums {:?}",nums);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    
    let t = scan.next::<usize>();
    for _ in (0..t){
        let n = scan.next::<usize>();
        let mut intervals : Vec<(i32,i32)> = (0..n).map(|_| {let l = scan.next(); let r = scan.next(); (l,r)}).collect();
        doit(&mut intervals);
    }

        Ok(())
    }

   
    
    
  

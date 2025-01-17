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

struct Solution {
    cache : HashMap<(usize,usize),bool>,
    n : usize,
}

impl Solution{

    fn new(n : usize) -> Solution {
        Solution {
            cache : HashMap::<(usize,usize),bool>::new(),
            n,
        }
    }
    // return true ifplayer1 can win, false if cannot
    pub fn player1_turn(&self,apos : usize, bpos : usize)->bool{
        if apos+2usize == bpos {
            return true;
        }
        else if apos+1 == bpos {
            return false;
        }
        
        return !self.player2_turn(apos+1,bpos);
    } 

    //return true is player2 can win from pos, false is canot
    pub fn player2_turn(&self, apos: usize, bpos : usize) ->bool {
        if apos+2usize==bpos {
            return true;
        }
        else if bpos == apos+1usize {
            return false;
        }
        return !self.player1_turn(apos,bpos-1);

    }
}

pub fn doit( grid : &mut Vec::<Vec::<i32>>, path : &str){
    let mut currr = 0usize;
    let mut currc = 0usize;

    let cvec: Vec<char> = path.chars().collect();
    
    let mut path  = path.chars().map(|ch| {
        if ch == 'D' {
            currr+=1;
        }
        else {
            currc+=1;
        }
        (currr, currc)
    }).collect::<Vec::<(usize,usize)>>();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    
    let t = scan.next::<usize>();
    for _ in (0..t){
        let n = scan.next::<usize>();
        let m = scan.next::<usize>();
        let path : String = scan.next::<String>();
        let mut grid = vec![vec![0 ; m];n];
        for r in (0..n){
            for c in (0..m) {
                grid[r][c] = scan.next::<i32>();
            }
        }
       
        doit(&mut grid);
        
    }

        Ok(())
    }



    
  

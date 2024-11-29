#[allow(unused_imports)]
use std::io::{self,stdin, stdout, BufWriter, Write,prelude::*, BufReader, Result};
use std::fs::File;
use std::env;
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::{DefaultHasher, RandomState},
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
    },
};

const MOD: usize = 1_000_000_007;
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

pub fn doit(ss : &str){
    //first go through entire line to see if its of the form
    // aba, and do sliding window, of size 3. if not, then you have a problem
    //return appropriate
    let sz = ss.len();
    if sz==1 {
        println!("-1");
        return;
    }
    if sz==2 {
        if ss.chars().nth(0)==ss.chars().nth(1) {
            println!("{}",ss);
            return;
        }
        
        println!("-1");
        return;
        
    }
    let mut tv:Vec<char> = ss.chars().collect();
    //check for adjacency problem
    for ii in 0..(sz-1){
        if tv[ii]==tv[ii+1]{
            //houston, we have a problem
            println!("{}",&ss[ii..(ii+2)]);
            return;
        }
    }
    //check for 3 strings
    for ii in 0..(sz-2){
        if tv[ii]!=tv[ii+2]{
            println!("{}",&ss[ii..(ii+3)]);
            return ;
        }
    }

    println!("-1");
    
}

fn main()-> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    /////////////////////////////
    /// use this when you are working on problem at home
    /// TODO: not sure if this reads whole file at a time or not
    ///////////////////////////// 
    /* 
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;

    println!("here is first line length:{}",line);

    let n = (line).trim().parse::<i32>().unwrap();
    println!("n is {}",n);
    
    
    let mut inputs: Vec<i32> = Vec::new();
    for line in reader.lines(){
        match line {
            Ok(ss) => doit(ss.as_str()),
            _ => None
        };

    };
    */

    //////////////////////////
    /// OR use this when you are about to submit
    /// this reads what is passed in to running executable
    //////////////////////////
    /// 
    
    let mut scan = Scanner::default();
    let n : usize = scan.next();

    for _ in 0..n {
        let word = scan.next::<String>();
        doit(&word.as_str());
    }
    
    
    

    //println!("the inputs are {:?}", inputs);
    //let t = inputs[0];

    
    /* 
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<usize>();

    for _ in 0..t {
        let n = scan.next::<usize>();
        let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

        a.sort();

        let mut b = VecDeque::with_capacity(n);
        b.push_front(*a.last().unwrap());
        let mut sum = b[0];
        let mut fail = false;
        for i in 0..n - 1 {
            if a[i] == sum {
                fail = true;
                //println!("ai : {} and sum {}", a[i], sum);
            }
            b.push_back(a[i]);
            sum += a[i];
        }

        if fail {
            println!("NO");
            continue;
        }
        println!("YES");

        println!("here is b {:?}",b);
    }
    */
    Ok(())
}

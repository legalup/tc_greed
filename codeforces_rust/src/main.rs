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

pub fn doit(n : i32){
    for i in 0..n {
        print!("{} ",1+2*i);
    }
    println!("");
}

fn main()-> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    /* 
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    */

    //for &line in reader.lines() {
    //    println!("{}", line?);
    //}
    

    /* 
    let mut inputs: Vec<i32> = Vec::new();
    for line in reader.lines(){
        match line {
            Ok(ss) => inputs.push(ss.parse::<i32>().unwrap()),
            _ => println!("got nutting honey") 
        }

    }
    */

    let mut scan = Scanner::default();
    let n : usize = scan.next();

    
    let mut inputs: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    

    //println!("the inputs are {:?}", inputs);
    //let t = inputs[0];

    for nn in 0..inputs.len(){
        doit(inputs[nn] as i32);
    }
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

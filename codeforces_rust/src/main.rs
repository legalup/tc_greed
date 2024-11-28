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

fn main()-> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
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

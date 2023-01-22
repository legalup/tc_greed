#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
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
pub fn printv<T: std::fmt::Display>(myvec: Vec<T>) {
    myvec.iter().for_each(|vv| print!("{vv} "));
}
pub fn printvd<T: std::fmt::Display>(myvec: VecDeque<T>) {
    myvec.iter().for_each(|vv| print!("{vv} "));
}

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

fn main() {
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

        printvd(b);
    }
}

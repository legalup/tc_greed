
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use core::fmt::Error;

static mut g_source : Option<io::Lines<io::BufReader<File>>> = None;

fn get_line() -> String {
    let mut line = String::new();
    while line.is_empty() {
        match g_source {
            None => {std::io::stdin().read_line(&mut line).unwrap();
            }
            Some(lines) => {
               let rez = lines.next();           
            }
        }
        
    }
    line.pop().unwrap();
    line
}

macro_rules! read_line {
        ( $($name:ident : $typ:ty),+ ) => {
            let line = get_line();
            let mut tokens = line.split_ascii_whitespace();
            $(
                let $name: $typ = tokens.next().unwrap().parse().unwrap();
            )+
            debug_assert_eq!(tokens.next(), None);
        };
    }

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_line_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    get_line()
        .split_ascii_whitespace()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .unwrap()
}

fn run() {
    read_line!(n: usize);
    let mut a: Vec<i32> = read_line_vec();
    let mut c1 = a.iter().filter(|x| **x == 1).count();
    let mut cm1 = a.iter().filter(|x| **x == -1).count();
    if c1 < cm1 {
        for i in 0..n {
            a[i] = -a[i];
        }
        (c1, cm1) = (cm1, c1);
    }
    assert!(c1 >= cm1);
    if (c1 - cm1) % 2 != 0 {
        println!("-1");
        return;
    }
    let mut prev = true;
    let mut ct = n;
    let mut join = vec![false; n];
    for i in 0..n {
        if c1 > cm1 && a[i] == 1 && !prev {
            prev = true;
            ct -= 1;
            join[i - 1] = true;
            c1 -= 2;
        } else {
            prev = false;
        }
    }
    println!("{}", ct);
    let mut i = 0;
    while i < n {
        if join[i] {
            println!("{} {}", i + 1, i + 2);
            i += 2;
        } else {
            println!("{} {}", i + 1, i + 1);
            i += 1;
        }
    }
}

fn main() {

   /*  if let Ok(lines) = read_lines("/home/lgalup/workspace/tc_greed/cf_rust/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    } */

    let poopy = read_lines("/home/lgalup/workspace/tc_greed/cf_rust/input.txt");
    if let Ok(a) = poopy{
        unsafe{
            g_source = Some(a);
        }
    }



   /*  read_line!(t: usize);
    for _ in 0..t {
        run();
    } */

    /* let values = vec![1, 2, 3, 4, 5];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                let next;
                match iter.next() {
                    Some(val) => next = val,
                    None => break,
                };
                let () = {
                    println!("{next}");
                };
            },
        };
        result
    } */
}

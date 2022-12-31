#![allow(dead_code, unused_macros, unused_imports)]
use std::{cell::{Cell, RefCell, UnsafeCell}, cmp::{Ordering, Reverse, max, min}, collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque, hash_map::{DefaultHasher, RandomState}}, error::Error, fmt::{Display, Write as FmtWrite}, hash::{BuildHasher, Hash, Hasher}, io::{BufWriter, Read, Stdin, Stdout, Write}, iter::{FromIterator, Peekable}, mem::swap, ops::*, process::exit, rc::Rc, str::{FromStr, from_utf8_unchecked}, time::{Duration, Instant}, convert::{TryInto, TryFrom}, marker::PhantomData};

const IO_BUF_SIZE: usize = 1 << 16;
type Input = Scanner<Stdin>;
type Output = BufWriter<Stdout>;
fn _init_input() -> Input { Scanner::new(std::io::stdin()) }
fn _init_output() -> Output { BufWriter::with_capacity(IO_BUF_SIZE, std::io::stdout()) }

#[repr(transparent)] struct Unsync<T>(T);
unsafe impl<T> Sync for Unsync<T> {}
 
type BadLazy<T> = Unsync<UnsafeCell<Option<T>>>;
impl<T> BadLazy<T> {
    const fn new() -> Self { Self(UnsafeCell::new(None)) }
}
 
static INPUT: BadLazy<Input> = BadLazy::new();
static OUTPUT: BadLazy<Output> = BadLazy::new();
 
fn inp<F: FnOnce(&mut Input) -> R, R>(f: F) -> R {
    unsafe { f((&mut *INPUT.0.get()).get_or_insert_with(_init_input)) }
}
fn out<F: FnOnce(&mut Output) -> R, R>(f: F) -> R {
    unsafe { f((&mut *OUTPUT.0.get()).get_or_insert_with(_init_output)) }
}

macro_rules! read {
    () => { read() };
    ($t: ty) => { read::<$t>() };
    ($t: ty, $($tt: ty),*) => { (read::<$t>(), $(read::<$tt>(),)*) };
    [$t: ty; $n: expr] => { read_vec::<$t>($n) };
}
macro_rules! println { 
    () => { out(|x| { let _ = writeln!(x); }) };
    ($exp: expr) => { out(|x| { let _ = writeln!(x, "{}", $exp); }) }; 
    ($fmt: expr, $($arg : tt )*) => { out(|x| { let _ = writeln!(x, $fmt, $($arg)*); }) }
}
macro_rules! print { 
    ($exp: expr) => { out(|x| { let _ = write!(x, "{}", $exp); }) }; 
    ($fmt: expr, $($arg : tt )*) => { out(|x| { let _ = write!(x, $fmt, $($arg)*); }) }
}

fn out_flush() { out(|x| { let _ = x.flush(); }); }

fn input_is_eof() -> bool { inp(|x| x.eof()) }
fn read_byte() -> u8 { inp(|x| x.byte()) }
fn read_bytes_no_skip(n: usize) -> Vec<u8> { inp(|x| x.bytes_no_skip(n)) }
fn read_bytes(n: usize) -> Vec<u8> { inp(|x| x.bytes(n)) }
fn read_bytes2(n: usize, m: usize) -> Vec<Vec<u8>> { inp(|x| x.bytes2(n, m)) }
fn read_token() -> Vec<u8> { inp(|x| x.token_bytes()) }
fn read_token_str() -> String { unsafe { String::from_utf8_unchecked(read_token()) } }
fn read_line() -> Vec<u8> { inp(|x| x.line_bytes()) }
fn read_line_str() -> String { unsafe { String::from_utf8_unchecked(read_line()) } }
fn read<T: FromStr>() -> T { read_token_str().parse::<T>().ok().expect("failed parse") }
fn read_vec<T: FromStr>(n: usize) -> Vec<T> { (0..n).map(|_| read()).collect() }
fn read_vec2<T: FromStr>(n: usize, m: usize) -> Vec<Vec<T>> { (0..n).map(|_| read_vec(m)).collect() }

struct Scanner<R: Read> {
    src: R,
    _buf: Vec<u8>,
    _pt: usize, // pointer
    _rd: usize, // bytes read
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(src: R) -> Scanner<R> {
        Scanner { src, _buf: vec![0; IO_BUF_SIZE], _pt: 1, _rd: 1 }
    }
 
    fn _check_buf(&mut self) {
        if self._pt == self._rd {
            self._rd = self.src.read(&mut self._buf).unwrap_or(0);
            self._pt = (self._rd == 0) as usize;
        }
    }
 
    // returns true if end of file
    fn eof(&mut self) -> bool {
        self._check_buf();
        self._rd == 0
    }
 
    // filters \r, returns \0 if eof
    fn byte(&mut self) -> u8 {
        loop {
            self._check_buf();
            if self._rd == 0 { return 0; }
            let res = self._buf[self._pt];
            self._pt += 1;
            if res != b'\r' { return res; }
        }
    }

    fn bytes_no_skip(&mut self, n: usize) -> Vec<u8> { (0..n).map(|_| self.byte()).collect() }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let res = self.bytes_no_skip(n);
        self.byte();
        res
    }
    fn bytes2(&mut self, n: usize, m: usize) -> Vec<Vec<u8>> { (0..n).map(|_| self.bytes(m)).collect() }
 
    fn token_bytes(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut c = self.byte();
        while c <= b' ' {
            if c == b'\0' { return res; }
            c = self.byte();
        }
        loop {
            res.push(c);
            c = self.byte();
            if c <= b' ' { return res; }
        }
    }
 
    fn line_bytes(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut c = self.byte();
        while c != b'\n' && c != b'\0' {
            res.push(c);
            c = self.byte();
        }
        res
    }
}

trait JoinToStr { 
    fn join_to_str(self, sep: &str) -> String;
    fn concat_to_str(self) -> String;
}
impl<T: Display, I: Iterator<Item = T>> JoinToStr for I { 
    fn join_to_str(mut self, sep: &str) -> String {
        match self.next() {
            Some(first) => {
                let mut res = first.to_string();
                while let Some(item) = self.next() {
                    res.push_str(sep);
                    res.push_str(&item.to_string());
                }
                res
            }
            None => { String::new() }
        }
    }
 
    fn concat_to_str(self) -> String {
        let mut res = String::new();
        for item in self { res.push_str(&item.to_string()); }
        res
    }
}
trait AsStr { fn as_str(&self) -> &str; }
impl AsStr for [u8] { fn as_str(&self) -> &str {std::str::from_utf8(self).expect("attempt to convert non-UTF8 byte string.")} }

macro_rules! veci {
    ($n:expr , $i:ident : $gen:expr) => {{
        let _veci_n = $n;
        let mut _veci_list = Vec::with_capacity(_veci_n);
        for $i in 0.._veci_n {
            _veci_list.push($gen);
        }
        _veci_list
    }};
    ($n:expr , $gen:expr) => { veci!($n, _veci_: $gen) }
}

fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x < y { y - x } else { x - y }
}

trait CommonNumExt {
    fn div_ceil(self, b: Self) -> Self;
    fn div_floor(self, b: Self) -> Self;
    fn gcd(self, b: Self) -> Self;
    fn highest_one(self) -> Self;
    fn lowest_one(self) -> Self;
    fn sig_bits(self) -> u32;
}

macro_rules! impl_common_num_ext {
    ($($ix:tt = $ux:tt),*) => {
        $(
            impl CommonNumExt for $ux {
                fn div_ceil(self, b: Self) -> Self {
                    let q = self / b; let r = self % b;
                    if r != 0 { q + 1 } else { q }
                }
                fn div_floor(self, b: Self) -> Self { self / b }
                fn gcd(self, mut b: Self) -> Self {
                    let mut a = self;
                    if a == 0 || b == 0 { return a | b; }
                    let shift = (a | b).trailing_zeros();
                    a >>= a.trailing_zeros();
                    b >>= b.trailing_zeros();
                    while a != b {
                        if a > b { a -= b; a >>= a.trailing_zeros(); }
                        else { b -= a; b >>= b.trailing_zeros(); }
                    }
                    a << shift
                }
                #[inline] fn highest_one(self) -> Self { 
                    if self == 0 { 0 } else { const ONE: $ux = 1; ONE << self.sig_bits() - 1 } 
                }
                #[inline] fn lowest_one(self) -> Self { self & self.wrapping_neg() }
                #[inline] fn sig_bits(self) -> u32 { std::mem::size_of::<$ux>() as u32 * 8 - self.leading_zeros() }
            }

            impl CommonNumExt for $ix {
                fn div_ceil(self, b: Self) -> Self {
                    let q = self / b; let r = self % b;
                    if self ^ b >= 0 && r != 0 { q + 1 } else { q }
                }
                fn div_floor(self, b: Self) -> Self { 
                    let q = self / b; let r = self % b;
                    if self ^ b < 0 && r != 0 { q - 1 } else { q }
                }
                fn gcd(self, b: Self) -> Self {
                    fn w_abs(x: $ix) -> $ux { (if x.is_negative() { x.wrapping_neg() } else { x }) as _ }
                    w_abs(self).gcd(w_abs(b)) as _
                }
                #[inline] fn highest_one(self) -> Self { (self as $ux).highest_one() as _ }
                #[inline] fn lowest_one(self) -> Self { self & self.wrapping_neg() }
                #[inline] fn sig_bits(self) -> u32 { std::mem::size_of::<$ix>() as u32 * 8 - self.leading_zeros() }
            }
        )*
    }
}
impl_common_num_ext!(i8 = u8, i16 = u16, i32 = u32, i64 = u64, i128 = u128, isize = usize);

trait ChMaxMin<T> {
    fn chmax(&mut self, v: T) -> bool;
    fn chmin(&mut self, v: T) -> bool;
}
impl<T: PartialOrd> ChMaxMin<T> for Option<T> {
    fn chmax(&mut self, v: T) -> bool { if self.is_none() || v > *self.as_ref().unwrap() { *self = Some(v); true } else { false } }
    fn chmin(&mut self, v: T) -> bool { if self.is_none() || v < *self.as_ref().unwrap() { *self = Some(v); true } else { false } }
}
impl<T: PartialOrd> ChMaxMin<T> for T {
    fn chmax(&mut self, v: T) -> bool { if v > *self { *self = v; true } else { false } }
    fn chmin(&mut self, v: T) -> bool { if v < *self { *self = v; true } else { false } }
}

// * end commons * //


#[allow(non_snake_case, non_upper_case_globals)]
fn main() {
    let num_cases: usize = read();
	
    for _case_num in 1..=num_cases {
        let n = read!(usize);
        let m = read!(usize);

        let mut A = (0..n).map(|_| Reverse(read!(i64))).collect::<BinaryHeap<_>>();
        let mut s = A.iter().map(|a| a.0).sum::<i64>();

        for _ in 0..m {
            let b = read!(i64);

            s += b - A.pop().unwrap().0;
            A.push(Reverse(b));
        }

        println!(s);
    }
 
    out_flush();
}
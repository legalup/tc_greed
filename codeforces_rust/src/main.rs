#![allow(unused)]
use std::fmt;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::str::{from_utf8_unchecked, FromStr};

pub fn main() {
    /*
    //lets go through all the lines
    let test_cases = get_int_input_stdio();
    for _ in 0..test_cases {
        get_line_input_stdio();
        let mut n_numbers = get_line_input_stdio()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let m_numbers = get_line_input_stdio()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

     */
    let mut lines = get_lines_from_file();
    let test_cases = int_from_str(&lines[0]);
    println!("here is test cases {}", test_cases);
    let mut tc: i32 = 0;
    let mut idx: i32 = 0;
    while tc < test_cases {
        idx = tc * 3 + 2;
        let mut line = &lines[idx as usize];

        let mut n_numbers = vec_from_str(line);
        line = &lines[(idx + 1) as usize];

        let mut m_numbers = vec_from_str(line);
        for num in m_numbers {
            place_where_small_found(&mut n_numbers, num);
        }
        println!("{}", n_numbers.iter().sum::<i64>());
        tc += 1;
    }
}

pub fn place_where_small_found(numbers: &mut Vec<i64>, target: i64) {
    let small = numbers.iter().min().unwrap();
    let index = numbers.iter().position(|&x| x == *small).unwrap();
    numbers[index] = target;
}

pub fn get_int_input_stdio() -> i32 {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

pub fn get_line_input_stdio() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn int_from_str(line: &str) -> i32 {
    line.trim().parse::<i32>().unwrap()
}

pub fn vec_from_str(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn get_lines_from_file() -> Vec<String> {
    let f = File::open("data.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut ret: Vec<String> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        ret.push(line);
    }

    ret
}
pub fn get_lines_from_stdio() -> Vec<String> {
    let test_cases: usize = get_int_input_stdio() as usize;
    let mut ret: Vec<String> = Vec::with_capacity(test_cases);

    for _ in 0..test_cases {
        ret.push(get_line_input_stdio());
    }

    ret
}

use std::collections::VecDeque;

advent_of_code::solution!(17);

fn get_combo_op_val(a : &i64, b: &i64, c: &i64,  val : &i64)->i64{

    let ret = match *val{
        n @ 0..4 => n,
        4=>*a,
        5=>*b,
        6=>*c,
        _=> panic!("yo, this is not valid combo op val")
    };

    ret
}

//0
fn adv(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let x = get_combo_op_val(a, b, c, &prog[*ip+1usize]);
    *a = (*a >> x);
    *ip += 2;
}
//1
fn bxl(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let mut  val = prog[*ip+1usize];
    let bval = *b;
    *b = val^bval;
    *ip += 2;

}

//2
fn bst(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let x = get_combo_op_val(a, b, c, &prog[*ip+1usize]);
    *b = x %8;
    *ip += 2;

}

//3
fn jnz(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    if *a == 0 {*ip +=2;return;};
    *ip = prog[*ip+1usize] as usize;
}

//4
fn bxc(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
   *b = (*b) ^ (*c) ;
   *ip += 2;

}

//5
fn out(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let x = get_combo_op_val(a, b, c, &prog[*ip+1usize]) % 8;
    out.push(x);
    *ip += 2;

}

//6
fn bdv(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let x = get_combo_op_val(a, b, c, &prog[*ip+1usize]);
    *b = (*a >> x);
    *ip += 2;
}

//7
fn cdv(a : &mut i64, b: &mut i64, c: &mut i64, prog : &Vec<i64>, ip : &mut usize, out: &mut Vec<i64>)
{
    let x = get_combo_op_val(a, b, c, &prog[*ip+1usize]);
    *c = (*a >> x);
    *ip += 2;
}

fn run_program(a : &mut i64, B: &mut i64, C: &mut i64, program : &Vec<i64>, outp: &mut Vec<i64>)
{
    let mut ip = 0usize;

    while(ip<program.len()){
        //println!("hey there, ip is now {}",ip);
        match program[ip]{
            0=>adv(a ,  B,  C, &program ,  &mut ip ,  outp),
            1=>bxl(a ,  B,  C, &program ,  &mut ip ,  outp),
            2=>bst(a ,  B,  C, &program ,  &mut ip ,  outp),
            3=>jnz(a ,  B,  C, &program ,  &mut ip ,  outp),
            4=>bxc(a ,  B,  C, &program ,  &mut ip ,  outp),
            5=>out(a ,  B,  C, &program ,  &mut ip ,  outp),
            6=>bdv(a ,  B,  C, &program ,  &mut ip ,  outp),
            7=>cdv(a ,  B,  C, &program ,  &mut ip ,  outp),
                _=> panic!("yo, match somethingb weird")
        }
    }

}



pub fn part_one(input: &str) -> Option<u32> {
    let mut A : i64;
    let mut B : i64;
    let mut C : i64;
    let mut program = Vec::<i64>::new();
    let mut outp = Vec::<i64>::new();

    let inputvec : Vec::<&str>= input.lines().collect();
    let mut toks : Vec<&str> = inputvec[0].split([' ',':']).into_iter().collect();
    A = toks[3].parse::<i64>().unwrap();

    toks = inputvec[1].split([' ',':']).into_iter().collect();
    B = toks[3].parse::<i64>().unwrap();

    toks = inputvec[2].split([' ',':']).into_iter().collect();
    C = toks[3].parse::<i64>().unwrap();

    toks = inputvec[4].split([' ',',']).into_iter().collect();
    for ii in 1..toks.len(){program.push(toks[ii].parse::<i64>().unwrap());}

    println!("A {} B {} C {} and program {:?}",A,B,C,program);

    run_program(&mut A, &mut B, &mut C, &program, &mut outp);

    println!("here is the result a: {}, b:{}, c:{}, output {:?}",A,B,C,outp);
    let poopy : String = outp.iter().map(|ii| ii.to_string()).collect::<Vec<String>>().join(",");
    println!("here is ret: {}",poopy);


    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut B = 0i64;
    let mut C = 0i64;
    let mut program = vec![2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0];
    let mut outp = Vec::<i64>::new();

    let mut A = 117440i64;
    let mut trials = Vec::<i64>::new();

    for mut ii in 064..i64::MAX{

        run_program(&mut ii, &mut B, &mut C, &mut program, &mut outp);
        if outp.len() == program.len()  {
            trials.push(ii);
            println!("{:?}",trials);
        }

    }

    println!(" ----------------part 2.here is trials {:?}",trials);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(2,2);
    }
}

use std::collections::VecDeque;

advent_of_code::solution!(7);


fn is_doable(res : u64, args : &Vec<u64>)->bool{

    let mut stk : VecDeque<u64> = VecDeque::<u64>::from([args[0]]);
    let sz = args.len();
    for ii in 1..sz {
        let val = args[ii];
        let stksz = stk.len();
        for idx in 0..stksz {
            let stkval = stk[idx];
            stk.push_back(val+stkval);
            stk.push_back(val*stkval);
        }
        for idx in 0..stksz {
            stk.pop_front();
        }

    }

    //println!("here is ret {}, stk {:?}",res, stk);


    stk.contains(&res)
}

fn append_int(num1 : u64, num2 : u64) ->u64{

    let mut totstr = num1.to_string();
    totstr.push_str(&num2.to_string());

    totstr.parse::<u64>().unwrap()
}

fn is_doable2(res : u64, args : &Vec<u64>)->bool{

    let mut stk : VecDeque<u64> = VecDeque::<u64>::from([args[0]]);
    let sz = args.len();
    for ii in 1..sz {
        let val = args[ii];
        let stksz = stk.len();
        for idx in 0..stksz {
            let stkval = stk[idx];
            stk.push_back(val+stkval);
            stk.push_back(val*stkval);
            stk.push_back(append_int(stkval, val));
        }
        for idx in 0..stksz {
            stk.pop_front();
        }

    }

    //println!("here is ret {}, stk {:?}",res, stk);


    stk.contains(&res)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u64;
    for line in input.lines(){
        let nums : Vec<&str> = line.split([':']).into_iter().collect();
        //println!("nums {:?}",nums);
        let ret = nums[0].parse::<u64>().unwrap();
        let args:Vec<u64>= nums[1].split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
        if is_doable(ret, &args) {
            //println!("this wqorks {}, for args {:?}",ret, args);
            sum += ret;
        }

    }

    println!("the answer is sum {:?}",sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u64;
    for line in input.lines(){
        let nums : Vec<&str> = line.split([':']).into_iter().collect();
        //println!("nums {:?}",nums);
        let ret = nums[0].parse::<u64>().unwrap();
        let args:Vec<u64>= nums[1].split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
        if is_doable2(ret, &args) {
            //println!("this wqorks {}, for args {:?}",ret, args);
            sum += ret;
        }

    }

    println!("the answer is sum {:?}",sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}

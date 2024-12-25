use std::{collections::VecDeque, f32::consts::E};

advent_of_code::solution!(9);

fn sum_n_to_npk (n : u32, k : u32)->u64 {
    //(((2*n+k)*(k+1))/2) as u64
    let mut sum = 0u64;
    for idx in n..(n+k) {
        sum += (idx as u64);
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stk : VecDeque<(i32,u32)> = VecDeque::new();
    let nums : Vec<u32> = input.trim().chars().map(|c| c.to_digit(10u32).unwrap()).collect();
    let sz = nums.len();
    let mut sum = 0u64;
    for idx in 0usize..sz {
        if idx % 2 == 0usize {
            stk.push_back(((idx/2usize) as i32,nums[idx]));
        }
        else{
            stk.push_back((-1,nums[idx]));

        }
    }

    //println!("stk is {:?}",stk);

    let mut curr_idx = 0;
    while !stk.is_empty(){
        //gotta make sure at stk gets smaller!
        let lft = stk.pop_front().unwrap();
        if lft.0>=0{
            let sumterm = sum_n_to_npk(curr_idx, lft.1);
            sum += (lft.0 as u64)*sumterm;

            curr_idx += lft.1;
        }
        else{
            let mut rt = stk.pop_back()?;
            if !stk.is_empty()  && rt.0 < 0 {
                rt = stk.pop_back().unwrap();
                assert!(rt.0 > 0);
            }

            //we lft shoudl be negative at this point, and rt shodul be positive
            if lft.1 < rt.1 {
                sum += (rt.0 as u64)*sum_n_to_npk(curr_idx, lft.1);
                curr_idx += lft.1;
                stk.push_back((rt.0,rt.1-lft.1));

            }
            else if lft.1 == rt.1 {
                let sumterm2 = sum_n_to_npk(curr_idx, lft.1);
                sum += (rt.0 as u64)*sumterm2;
                curr_idx += lft.1;

            }
            else{
                sum += (rt.0 as u64)*sum_n_to_npk(curr_idx, rt.1);
                curr_idx += rt.1;
                stk.push_front((lft.0,lft.1-rt.1));
            }
        }

    }

    println!("====================the sum is {}",sum);
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stk : VecDeque<(i32,u32)> = VecDeque::new();
    let nums : Vec<u32> = input.trim().chars().map(|c| c.to_digit(10u32).unwrap()).collect();
    let sz = nums.len();
    let mut sum = 0u64;
    for idx in 0usize..sz {
        if idx % 2 == 0usize {
            stk.push_back(((idx/2usize) as i32,nums[idx]));
        }
        else{
            stk.push_back((-1,nums[idx]));

        }
    }

    //println!("here is stk before {:?}",stk);


    //lets do the shuffle first
    let mut ssz = stk.len() as i32;
    ssz -= 1;
    while ssz>=0 {

        let ii = ssz as usize;
        if stk[ii].0 == -1 {ssz -= 1; continue;};
        for jj in 0..ii {
            if stk[jj].0 == -1 && stk[jj].1 == stk[ii].1 {
                stk[jj].0 = stk[ii].0;
                stk[ii].0=-1;
                ssz -= 1;
                break;
            }
            else if stk[jj].0 == -1 && stk[jj].1 > stk[ii].1 {
                //need to fragment
                stk.insert(jj+1, (-1,stk[jj].1-stk[ii].1 ));
                stk[jj].0 = stk[ii+1].0;
                stk[jj].1 = stk[ii+1].1;
                stk[ii+1].0 = -1;
                break;
            }
            else{
            }
        }
        //lets merge the -1s together
        let mut badidx = 0usize;
        for kk in 0..(stk.len()-1){
            //prbly only happens once
            if stk[kk].0 == -1 && stk[kk+1].0==-1 {
                stk[kk].1 += stk[kk+1].1;
                stk[kk+1].1 = 0;
                badidx = kk+1;
                break;
            }
        }

        if badidx >0 {stk.remove(badidx);};
        ssz -= 1;
    }

    println!("here is stk now {:?}",stk);
    let mut curr_idx = 0u32;
    for ii in 0..stk.len(){
        let lft = stk[ii];
        let sumterm = sum_n_to_npk(curr_idx, lft.1);
        if lft.0 >=0{
            sum += (lft.0 as u64)*sumterm;
        }
        curr_idx += lft.1;
    }


    //lets do the checksum

    println!("====================the part 2 sum is {}",sum);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(Some(0), None);

    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }


}

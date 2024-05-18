/*
Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
frequency
of at least one of the chosen numbers is different.

The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

 
*/

use std::collections::VecDeque;
use std::cmp::Reverse;
use std::collections::HashMap;


impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combos : VecDeque<Vec<i32>> = VecDeque::new();
        combos.push_back(vec![0]);

        let mut ret : Vec<Vec<i32>> = vec![];

        
        for candy in candidates{
            let sz = combos.len();

            for ii in 0..sz {
                if let Some(mut vecky) = combos.pop_front() {
                    combos.push_back(vecky.to_vec()); //dont add this in
                    while vecky[0]+candy<=target{
                        vecky[0] += candy;
                        vecky.push(candy);
                        

                        let mut nuv = vecky.clone();
                        
                        if vecky[0]<target {
                            combos.push_back(nuv);
                            
                        }
                        else{
                            nuv.remove(0);
                            ret.push(nuv);
                        }
                    }
                }
            } 
        }
       

        ret
    }
}
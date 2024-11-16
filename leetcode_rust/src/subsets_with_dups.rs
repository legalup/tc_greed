/*
Given an integer array nums that may contain duplicates, return all possible
subsets
(the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.

 

Example 1:

Input: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]

Example 2:

Input: nums = [0]
Output: [[],[0]]

 

Constraints:

    1 <= nums.length <= 10
    -10 <= nums[i] <= 10



*/

use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut outty = Vec::new();
        let mut mt = Vec::<i32>::new();
        outty.push(mt);

        //lets make a dictionary. but how
        let freqs = nums
          .iter()
          .copied()
          .fold(HashMap::new(), |mut mappy, val|{
              mappy.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              mappy
          });

        //println!("here is freq {:?}",freqs);
        for (k,v) in freqs{
            let sz = outty.len();
            let mut nuoutty = outty.clone();
            for vv in 0..sz {
                let mut cc = outty[vv].clone();
                //outty.push(cc.clone());
                for kk in 0..v {
                    cc.push(k);
                    outty.push(cc.clone());
                }
            }
        }

        outty
    }
}
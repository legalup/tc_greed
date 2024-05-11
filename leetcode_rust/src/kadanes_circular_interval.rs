/*
Given a circular integer array nums of length n, return the maximum possible sum of a non-empty subarray of nums.

A circular array means the end of the array connects to the beginning of the array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i - 1 + n) % n].

A subarray may only include each element of the fixed buffer nums at most once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k1, k2 <= j with k1 % n == k2 % n.
*/


use std::cmp::max;


pub fn max_sub_array(nums: &Vec<i32>) -> i32 {
    nums.iter().fold((i32::MIN, 0), |(maxsum, cursum), &num| {
        let cursum = max(cursum + num, num);
        (max(maxsum, cursum), cursum)
    }).0
}



impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {

        let &mx = nums.iter().max().unwrap();
        if mx<=0 {
            return mx
        }

        let msa = max_sub_array(&nums);
        let mut v: Vec<i32> = nums.iter().map(|x| -x).collect();

        let mut summy : i32 = nums.iter().sum();
        let msa2 = summy +max_sub_array(&v);

        //println!("msa:{} and other:{}", msa, msa2);
        msa.max(msa2)
      
    }
}
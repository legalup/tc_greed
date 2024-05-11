/*
Given an integer array nums, find the
subarray
with the largest sum, and return its sum.

*/


use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((i32::MIN, 0), |(maxsum, cursum), &num| {
            let cursum = max(cursum + num, num);
            (max(maxsum, cursum), cursum)
        }).0
    }
}
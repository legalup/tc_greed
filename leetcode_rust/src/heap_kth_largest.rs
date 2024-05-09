/*
Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

Can you solve it without sorting?


*/

use std::collections::BinaryHeap;
use std::cmp::Reverse;


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heapy : BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let kus = k as usize;
        for &ii in nums.iter(){
            if heapy.len()<kus {
                heapy.push(Reverse(ii));
            }
            else {
                let Reverse(vv) = heapy.peek().unwrap();
                if *vv < ii {
                    heapy.push(Reverse(ii));
                    heapy.pop();
                }
            }
        }

        let Reverse(vv) = heapy.peek().unwrap();
        *vv
    }
}
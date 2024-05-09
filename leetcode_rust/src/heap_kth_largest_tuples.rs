
/*
You are given two integer arrays nums1 and nums2 sorted in non-decreasing order and an integer k.

Define a pair (u, v) which consists of one element from the first array and one element from the second array.

Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.

*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
       
        let (m, n) = (nums1.len(), nums2.len());
        let mut res = vec![];

        for i in 0..m {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0 as usize)));
        }

        for counter in 0..k {
            if let Some(Reverse((_sum, i, j))) = heap.pop() {
                res.push(vec![nums1[i], nums2[j]]);

                if j + 1 < n {
                    heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
                }
            }

        }

        res
    }
}


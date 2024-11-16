/*
a fabulous one liner to convert a vector of ints to a frequency. using hashmap

*/

use std::collections::HashMap;


impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut outty = Vec::new();
        let mut mt = Vec::<i32>::new();
        outty.push(mt);

        //here is how to make a frequency
        let freqs = nums
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });

        println!("here is freq {:?}",freqs);

        outty
    }

}
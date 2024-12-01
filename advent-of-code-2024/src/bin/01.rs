advent_of_code::solution!(1);

use std::num;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    let mut nums1 : Vec<i32> = Vec::new();
    let mut nums2 : Vec<i32> = Vec::new();

    for line in input.lines(){
        let mut nums : Vec<i32> = line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        nums1.push(nums[0]);
        nums2.push(nums[1]);

    }

    nums1.sort();
    nums2.sort();

    for idx in 0..nums1.len(){
        sum += (nums1[idx]-nums2[idx]).abs() as u32;
    }


    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut nums1 : Vec<i32> = Vec::new();
    let mut nums2 : Vec<i32> = Vec::new();

    for line in input.lines(){
        let mut nums : Vec<i32> = line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        nums1.push(nums[0]);
        nums2.push(nums[1]);

    }

    let freqs = nums2
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });

    let mut sum = 0i32;
    for nn in nums1 {
        if freqs.contains_key(&nn) {
            sum += nn*freqs[&nn];
        }
    }


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

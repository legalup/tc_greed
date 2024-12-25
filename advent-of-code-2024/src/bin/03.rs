advent_of_code::solution!(3);
use std::f32::consts::E;

use regex::Regex;

fn get_sum(line : &str)->u32 {
    let mut sum = 0i32;
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let dates: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        for date in dates {
            let nums : Vec<&str> = date.split(['(',')',',']).into_iter().collect();
            let num1 = nums[1].parse::<i32>().unwrap();
            let num2 = nums[2].parse::<i32>().unwrap();
            sum += num1*num2;
        }
    sum as u32
}

pub fn part_one(input: &str) -> Option<u32> {

    Some(get_sum(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let dont = "don't()";
    let doo = "do()";
    let donts :Vec<&str> = input.split(dont).collect();
    //println!("here is donts {:?}",donts);
    let mut sum = 0u32;
    sum += get_sum(donts[0]);
    for ii in 1..donts.len() {
        let doos:Vec<&str> = donts[ii].split(doo).collect();
        for jj in 1..doos.len(){

            //println!("here is splits of doos {:?}",doos[1]);
            sum += get_sum(doos[jj]);

        }
    }
    Some(sum)
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

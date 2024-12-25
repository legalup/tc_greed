use std::{collections::{HashMap, HashSet}, ffi::IntoStringError};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules : Vec<Vec<&str>> = Vec::new();
    let mut keys = HashSet::<&str>::new();
    let mut isupdates = false;
    let mut sum = 0u32;
    for line in input.lines() {
        if line.len() == 0 {
            isupdates = true;
            //println!("here is rules {:?}",rules);
            continue;
        }
        if !isupdates {
            let rule : Vec<&str> = line.split('|').into_iter().collect();
            rules.push(vec![rule[0],rule[1]]);
        }
        else{
            //actually run this on the update
            let update : HashSet<&str> = line.split(',').collect();
            let mut isordered = true;

            for rule in rules.iter(){
                if update.contains(rule[0]) && update.contains(rule[1]){
                    let idx1= line.find(rule[0]);
                    let idx2= line.find(rule[1]);
                    if idx2.unwrap()<idx1.unwrap() {
                        isordered = false;
                        break;
                    }
                }
            }


            if isordered {
                //println!("this is ordered, i think {:?}",line);
                //lets find middle one
                let sss: Vec<&str> = line.split(',').collect();
                let sz = (sss.len()-1usize)/2usize;
                //println!("here is sz {}",sz);
                sum += sss[sz].parse::<u32>().unwrap();


            }
        }
    }



    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules : Vec<Vec<&str>> = Vec::new();
    let mut keys = HashSet::<&str>::new();
    let mut isupdates = false;
    let mut sum = 0u32;
    for line in input.lines() {
        if line.len() == 0 {
            isupdates = true;
            //println!("here is rules {:?}",rules);
            continue;
        }
        if !isupdates {
            let rule : Vec<&str> = line.split('|').into_iter().collect();
            rules.push(vec![rule[0],rule[1]]);
        }
        else{
            //actually run this on the update
            let update : HashSet<&str> = line.split(',').collect();
            let mut isordered = true;

            let mut badrules : Vec<Vec<&str>> = Vec::new();
            for rule in rules.iter(){
                if update.contains(rule[0]) && update.contains(rule[1]){
                    badrules.push(rule.clone());
                    let idx1= line.find(rule[0]);
                    let idx2= line.find(rule[1]);
                    if idx2.unwrap()<idx1.unwrap() {
                        isordered = false;
                        //break;
                    }
                }
            }


            if !isordered {
                //println!("this is ordered, i think {:?}",line);
                //lets find middle one
                //println!("here is badrules {:?}",badrules);
                let mut sss: Vec<&str> = line.split(',').collect();
                let (mut idx0,mut idx1) = (0usize, 0usize);
                let mut gotchanged = true;
                while gotchanged {
                    gotchanged = false;
                    for rule in badrules.iter(){
                        for idx in 0..sss.len(){
                            if sss[idx] == rule[0] {
                                idx0=idx;

                            }
                            else if sss[idx]==rule[1] {
                                idx1=idx;
                            }


                        }
                        //println!("rule is {:?}",rule);
                        //println!("here is the before ordered sss {:?}",sss);
                        if idx0>idx1{
                            gotchanged = true;
                            sss[idx0] = rule[1];
                            sss[idx1] = rule[0];
                        }
                        //println!("here is the newly ordered sss {:?}",sss);
                    }
                }
                //println!("here is the newly ordered sss {:?}",sss);

                let sz = (sss.len()-1usize)/2usize;
                //println!("here is sz {}",sz);
                sum += sss[sz].parse::<u32>().unwrap();

            }
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

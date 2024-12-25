use std::f32::consts::E;

advent_of_code::solution!(2);



fn is_safe(report : &Vec<i32>)->bool{

    let sz = report.len() -1usize;
    let diffs : Vec<i32> = (0..sz).into_iter().map(|x| {report[x]-report[x+1]}).collect();

    //println!("here is diffs {:?}",diffs);

    //first check they are all the same sign
    if diffs[0]>0 {
        for ii in (0..sz){
            if diffs[ii]<=0 || diffs[ii].abs()>3{
                //println!("which is unsafe. is increasing. ii is {}",ii);
                return false;
            }
        };
    }
    else if diffs[0]<0 {
        for ii in (0..sz){
            if diffs[ii]>=0 || diffs[ii].abs()>3{
                //println!("which is unsafe. is decreasing. ii is {}",ii);
                return false;
            }
        };

    }
    else {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut numsafe = 0u32;

    for line in input.lines(){
        let report : Vec<i32>= line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        if is_safe(&report) {
            numsafe += 1;
        }

    }
    Some(numsafe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numsafe = 0u32;

    for line in input.lines(){
        let report : Vec<i32>= line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        if is_safe(&report) {
            numsafe += 1;
        }
        else{
            for ii in 0..report.len(){
                let mut cpvec = report.clone();
                cpvec.remove(ii);
                if is_safe(&cpvec){
                    numsafe += 1;
                    break;
                }
            }
        }

    }
    Some(numsafe)
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

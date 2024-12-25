use std::collections::HashMap;

advent_of_code::solution!(11);


fn next( nummy : &mut Vec<u64>){
    let mut sz = nummy.len();
    let mut idx = 0usize;


    while idx < sz {
        let numstr = nummy[idx].to_string();
        //if 0, convert to 1
        if nummy[idx] == 0 {
            nummy[idx] = 1;
        }
        else if (numstr.len() % 2) == 0 {
            let s1 = &numstr[0..(numstr.len()/2)];
            let s2 = &numstr[(numstr.len()/2)..numstr.len()];

            //println!("s1 is {} and s2 is {}", s1, s2);
            let n1 = s1.parse::<u64>().unwrap();
            let n2 = s2.parse::<u64>().unwrap();
            nummy[idx] = n1;
            nummy.insert(idx+1, n2);
            sz += 1usize;
            idx += 1usize;
        }
        else{
            nummy[idx] *= 2024u64;
        }
        idx += 1usize;

    }
}

fn get_val(cache : &mut HashMap<(u64,u64),u64>, val : u64, time : u64)->u64{
    if cache.contains_key(&(val,time)) {return cache[&(val,time)]};

    let mut nums = vec![val];

    if time == 2u64 {
        next(&mut nums);
        next(&mut nums);
        let sum = nums.len() as u64;
        cache.insert((val,time), sum);
        return sum;
    }
    if time == 1u64 {
        next(&mut nums);
        let sum = nums.len() as u64;
        cache.insert((val,time), sum);
        return sum;
    }

    let nutime = time/3u64;
    for ii in 0..nutime {
        next(&mut nums);
    }

    let mut sum = 0u64;
    let smallertime = time- nutime;

    if time == smallertime {
        println!("this is fucked. time {} and smalelrtime {}",time,smallertime);
    }
    assert!(smallertime != time);
    for nn in nums {
        sum += get_val(cache, nn,time-nutime);
    }

    cache.insert((val,time), sum);

    sum

}



pub fn part_one(input: &str) -> Option<u32> {

    let mut nums : Vec::<u64> = input.split_whitespace().into_iter().map(|w| w.parse::<u64>().unwrap()).collect();

    println!("here is nums {:?}",nums);

    let mut cnums = nums.clone();

    /*
    for ii in 0..30{
    next(&mut nums);

    //println!("here is nums afer next {:?}",nums);
    }
    */

    println!("the answer doing brut force is {}",nums.len());

    let mut cache : HashMap::<(u64,u64),u64> = HashMap::<(u64,u64),u64>::new();

    let mut sum = 0u64;
    for nn in cnums{
        sum += get_val(&mut cache, nn, 75);
    }

    println!(" yo. answer doing caching is {}",sum);
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

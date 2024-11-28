advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {

    let mut sum = 0u32;

    for line in input.lines(){
        let charbuf:Vec::<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        //let diggy : u32 = charbuf[0]*10u32 + charbuf.last().unwrap();
        //sum +=diggy;
        //println!("here is diggy now {}",diggy);


    }

    println!("about to return this {}",sum);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut intys: HashMap<&str, u32> = HashMap::new();
    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");
    let four = String::from("four");
    let five = String::from("five");
    let six = String::from("six");
    let seven = String::from("seven");
    let eight = String::from("eight");
    let nine = String::from("nine");
    let mut vecstrs = vec![one,two,three,four,five,six,seven,eight,nine];
    for d in 1..=9 {
        vecstrs.push((d).to_string());
    }
    for d in 0..9 {
        intys.insert(vecstrs[d].as_str(),(d+1) as u32);
    }
    for d in 9usize..18usize {
        intys.insert(vecstrs[d].as_str(),(d-8) as u32);
    }

    println!("here is vecstrs {:?}",vecstrs);
    println!("and here is hapmap {:?}",intys);

    let mut sum = 0u32;

    for line in input.lines(){

        let mut smallest_match = (line.len(),"poopy");
        let mut biggest_match = (0usize,"poopy");
        //println!("here is line {} ",line);
        for pat in vecstrs.iter(){
            let itty:Vec<(usize,&str)> = line.match_indices(pat.as_str()).collect();
            if itty.len()>0 {
                if itty[0].0 <= smallest_match.0{
                    smallest_match = itty[0];
                }
                if itty.last().unwrap().0 >= biggest_match.0{
                    biggest_match = *itty.last().unwrap();
                }
            }

        }
        assert!(smallest_match.1 != "poopy");
        assert!(vecstrs.len() == 18);
        assert!(intys.len() == 18);

        //println!("here is smallest {:?} and here is biggest {:?}", smallest_match,  biggest_match);
        let diggy = intys[smallest_match.1] * 10u32 + intys[biggest_match.1];
        //println!("here is line {} and here is number {}",line, diggy);
        sum +=diggy;
    }

    Some(sum)


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

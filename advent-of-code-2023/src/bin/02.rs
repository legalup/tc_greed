advent_of_code::solution!(2);
use std::collections::HashMap;

pub fn game2hashmap(linny : &str)->HashMap<&str,u32>{
    let words : Vec<&str> =  linny.split(&[' ',','][..]).filter(|x|x.len()>0).collect();
    //println!("in game 2 hashamp, here is my words {:?}",words);
    let mut hm: HashMap<&str, u32> = HashMap::new();
    for mut ii in (0..words.len()).step_by(2){
        //println!("here is word about to parse here {}",words[ii]);
        let inty = words[ii].parse::<u32>().unwrap();
        ii +=1;
        hm.insert(words[ii],inty);
    }

    hm
}

pub fn line2hashmap(linny : &str)->(u32, HashMap<&str,u32>){
    let words : Vec<&str> =  linny.split(&[';',':'][..]).collect();
    //println!("the words of the line are {:?}",words);
    let mut hm: HashMap<&str, u32> = HashMap::new();
    //first lets get the index
    let preamble : Vec<&str> = words[0].split_ascii_whitespace().collect();
    let idx = preamble[1].parse::<u32>().unwrap();
    for ii in 1..words.len(){
        let tmphm = game2hashmap(words[ii]);

        for key in tmphm.keys() {
            let mut v = tmphm[key];
            if hm.contains_key(key){
                v = std::cmp::max(v,hm[key]);
            }
            hm.insert(key,v);
        }
    }
    //println!("for {} here is the max hashmap {:?}",idx,hm);
    (idx, hm)

}

pub fn part_one(input: &str) -> Option<u32> {

    let maxhm = HashMap::from([("red",12u32),("green",13u32),("blue",14u32)]);
    let mut sum = 0u32;
    for line in input.lines(){
        let (idx,hm) = line2hashmap(line);
        let mut is_possible = true;
        for key in hm.keys(){
            if !maxhm.contains_key(key) {
                is_possible=false;
            }
            else if maxhm[key]<hm[key] {
                is_possible=false;
                //println!("for {}, hm is {}",key,hm[key]);
            }
        };

        if is_possible{
            sum += idx;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    for line in input.lines(){
        let (idx,hm) = line2hashmap(line);
        sum += hm.values().fold(1u32,|mut poopy,val| {poopy*=val; poopy});
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

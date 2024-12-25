advent_of_code::solution!(19);
use std::collections::HashMap;
use std::cmp;

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_end_of_word
    }
}

struct Solver {
    forest:  HashMap<char, Trie>,
    maxlength : usize,
}
impl Solver {
    fn new(forest : HashMap<char, Trie>, maxlength : usize) -> Solver {
        Solver {
            forest,
            maxlength,
        }
    }

    //answers if werd[idx..] is possible. yeap.
    fn is_possible(&mut self, werd : &str)->bool{

        if werd.is_empty() {return false};
        let ch = werd.chars().nth(0).unwrap();
        if !self.forest.contains_key(&ch) {return false;};
        if werd.len()==1usize {return true;}
        let lenny = cmp::min(self.maxlength,werd.len());
        for ii in 1..=lenny{
            let prefixexists = self.forest[&ch].contains(&werd[0..ii]);

            if werd[ii..].len()==0 {return prefixexists};
            if prefixexists {
                let ispossible = self.is_possible(&werd[ii..]);

                if ispossible {return true;}
            }
        }

        false
    }


}

fn numways<'a>(maxlength : &usize, cache :  &mut HashMap::<&'a str,u64>, forest:  &HashMap<char, Trie>,werd : &'a str)->u64{

    if cache.contains_key(werd) {return cache[werd]};

    let mut ret = 0u64;
    if werd.is_empty() {return 0u64};
    let ch = werd.chars().nth(0).unwrap();
    if !forest.contains_key(&ch) {return 0u64};
    if werd.len()==1usize {
        if forest[&ch].contains(werd) { return 1u64}
        else {return 0u64};

    }
    let lenny = cmp::min(*maxlength,werd.len());
    for ii in 1..=lenny{
        let prefixexists = forest[&ch].contains(&werd[0..ii]);
        //println!("werd is {} this prefix exists {}",werd, &werd[0..ii]);

        if prefixexists {
            if werd[ii..].len()==0 {ret += 1u64}
            else{

                let nws = numways(maxlength, cache, forest, &werd[ii..]);
                //println!("for {} and prefix {} i get this many {} for the remainder {}",werd,&werd[0..ii],nws,&werd[ii..]);
                ret+=nws;

            }
        }
    }
    cache.insert(&werd,ret);
    //cache.entry(werd).and_modify(|f| *f+=(ret+1u64));
    ret
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let sec1 = sections.next().unwrap();
    let sec2 = sections.next().unwrap();
    let mut towels : Vec::<&str>= sec1.split([',',' ']).filter(|&x| {x.len()>0}).collect();
    let mut testwords = Vec::<&str>::new();
    let mut cache = HashMap::<&str,u64>::new();

    for line in sec2.lines(){
        testwords.push(line);
    }


    //println!("here is towels {:?}", towels);
    //println!("here is testwodss {:?}", testwords);

    let mut forest : HashMap<char, Trie> = HashMap::<char, Trie>::new();

    let mut maxlen = 0usize;

    //populating forest
     for ww in towels {
        if maxlen < ww.len() {maxlen = ww.len()};
        let ch = ww.chars().nth(0).unwrap();
        if !forest.contains_key(&ch) {forest.insert(ch, Trie::new());}
        forest.entry(ch).and_modify(|f| f.insert(ww));
    }

    let mut sum = 0u64;
    for werd in testwords.clone(){
        //let werd = String::from("brwrr");
        //let isok = solvy.is_possible(&werd);
        let nws = numways(&maxlen, &mut cache, &forest,werd);
        //println!("{} has this many ways {}",werd,nws);
        //if isok {println!("================is possible: {}",werd)}
        //else {println!("=================is NOT possible: {}",werd)};

        //if isok {sum +=1;}
        sum+=nws;
    }

    let mut solvy = Solver::new(forest, maxlen);
    for werd in testwords{
        let ispossible = solvy.is_possible(werd);

        if !ispossible {assert!(cache[werd]==0u64)};
        if cache[werd]==0u64  {assert!(!ispossible)};

    }

    println!(".........the numways is {}",sum);


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

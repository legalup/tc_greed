/*
Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
*/


use std::vec::Vec;

struct Generator
{
    occ : Vec<i32>,
}

impl Generator {
    pub fn next(&mut self)->bool {
        let mut ret = false;
        let mut sum=0usize;
        for idx in (0..(self.occ.len()-1)){
            sum += self.occ[idx] as usize;
            if self.occ[idx]<self.occ[idx+1] {
                
                (self.occ[idx],self.occ[idx+1]) = (1,0);
                for i in 0..sum {
                    self.occ[idx-1-i]=1;
                }
                
                return true;
            }
            else{
                self.occ[idx] = 0;
            }
            
        }
        false
    }

    fn symbolify(&self)->Vec<i32>
    {
        self.occ.iter().enumerate().filter_map(|(i,x)| {
            if *x > 0i32 {
                Some((i+1usize) as i32)
            }else {
                None
            }}).collect()
    }
}

impl Solution {

    

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combos : Vec<Vec<i32>> = Vec::new();
        
        let mut occ = vec![0i32;n as usize];
        for i in 0..k {
            occ[(n-1-i) as usize] = 1;
        }
 
        let mut gen = Generator{occ: occ};
        combos.push(gen.symbolify());
        while gen.next() {
            combos.push(gen.symbolify());
        }
       

        combos
    }
}
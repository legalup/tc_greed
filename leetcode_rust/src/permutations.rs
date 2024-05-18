
/*
Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
*/


impl Solution {

    fn dfs(nummy : &Vec<i32>, perms : &mut Vec<Vec<i32>>, v : &mut Vec<i32>, occ : &mut Vec<u32>)
    {
        let n = occ.len();

        if v.len() == n {
            perms.push(v.clone());
            return;
        }

        for i in 0..n {
            if occ[i]==0 {
                occ[i] = 1;
                v.push(nummy[i]);
                Self::dfs(nummy, perms,v,occ);
                v.pop();
                occ[i]=0;
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut occ = vec![0;nums.len()];
        let mut ret : Vec<Vec<i32>> = Vec::new();
        let mut v : Vec<i32> = Vec::new();
        Self::dfs(&nums, &mut ret,&mut v,&mut occ);

        ret
        
    }
}
/*
Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

*/


use std::collections::HashSet;

impl Solution {

    fn nbrs(grid: &Vec<Vec<char>>, sz : &(usize,usize),tup : &(usize,usize))->Vec<(usize,usize)>
    {
        let (r,c) = *tup;
        let mut ret : Vec<(usize,usize)> = vec![];

        if r>0 && grid[r-1][c] == '1' {ret.push((r-1,c))};
        if c>0 && grid[r][c-1] == '1' {ret.push((r,c-1))};
        if r+1<sz.0 && grid[r+1][c] == '1' {ret.push((r+1,c))};
        if c+1<sz.1 && grid[r][c+1] == '1' {ret.push((r,c+1))};
        
        ret
        
    }

    fn dfs(grid: &Vec<Vec<char>>, sz : &(usize,usize),used : &mut HashSet<(usize,usize)>, curr_comp : &mut HashSet<(usize,usize)>, vert:(usize,usize) ){
        curr_comp.insert(vert.clone());
        let mnbrs =Self::nbrs(grid,sz,&vert);
        used.insert(vert);
        for nbr in mnbrs{
            if !used.contains(&nbr){ //have to make sure we dont go on 4evuh
                Self::dfs(grid,sz,used, curr_comp, nbr);
            }
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {

        let mut used = HashSet::new();
        let mut comp = HashSet::new();
        let sz = (grid.len(),grid[0].len());

        let mut numcomps = 0i32;
        for r in 0..sz.0 {
            for c in 0..sz.1 {
                if grid[r][c] == '0' {continue;};
                let vert = (r,c);
                if !used.contains(&vert){
                    comp.clear();
                    Self::dfs(&grid,&sz, &mut used,&mut comp,vert);
                    numcomps += 1;
                    
                } 
            }
        }

        numcomps
        
    }
}
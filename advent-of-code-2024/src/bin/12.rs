use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

fn dist(tup1 : &(i32,i32),tup2: &(i32,i32)) -> u32{
    (i32::abs(tup1.0-tup2.0)+ i32::abs(tup1.1-tup2.1)) as u32
}

fn dfs(board : & Vec::<Vec::<char>>, used : &mut HashSet::<(usize,usize)>, comp : &mut HashSet::<(usize,usize)>,
vert : &(usize,usize) ){

    used.insert(vert.clone());
    comp.insert(vert.clone());
    let numrows = board.len();
    let numcols = board[0].len();
    let ch = board[vert.0][vert.1];

    //examine nbrs of vert
    //println!("in dfs. here is board {:?}",board);

    let ur = vert.0;
    let uc = vert.1;

    if ur>0usize && !used.contains(&(ur-1usize,uc)) && board[ur-1usize][uc]==ch {dfs(board,used,comp,&(ur-1usize ,uc))};
    if uc>0usize && !used.contains(&(ur,uc-1usize)) && board[ur][uc-1usize]==ch {dfs(board,used,comp,&(ur ,uc-1usize))};
    if ur < (numrows-1usize) && !used.contains(&(ur+1usize,uc)) && board[ur+1usize][uc]==ch {dfs(board,used,comp,&(ur+1usize ,uc))};
    if uc < (numcols-1usize) && !used.contains(&(ur,uc+1usize)) && board[ur][uc+1usize]==ch {dfs(board,used,comp,&(ur,uc+1usize))};

}


  fn find_comps(board : &mut Vec::<Vec::<char>>)->u32{
    let mut used = HashSet::<(usize,usize)>::new();
    let mut comp = HashSet::<(usize,usize)>::new();

    let numrows = board.len();
    let numcols = board[0].len();
    let mut sum = 0u32;

    for r in 0..numrows {
        for c in 0..numcols{
            if !used.contains(&(r,c)){
                comp.clear();
                dfs(board, &mut used, &mut comp, &(r,c));
                //println!("yo, here is a comp {:?} of size {}",comp,comp.len());
                //let p = get_price(&numrows, &numcols, &mut comp);
                let p = get_price2(&board, &numrows, &numcols, &mut comp);
                //println!("here is  size {} perimeter {} and char is {}",comp.len(),p,board[r][c]);
                sum += p*(comp.len() as u32);

            }
        }
    }

    sum
  }

  fn get_price(numrows : &usize, numcols : &usize, comp : &mut HashSet::<(usize,usize)>)->u32{

    //lets find the perimeter
    let mut perimeter = 0u32;
    for vert in comp.iter(){
        let &(r,c) = vert;
        if r==0usize {perimeter+=1}
        else if !comp.contains(&(r-1usize,c)){perimeter+=1;}

        if c==0usize {perimeter+=1;}
        else if !comp.contains(&(r,c-1usize)){perimeter+=1;}

        if r==numrows-1usize {perimeter+=1;}
        else if !comp.contains(&(r+1usize,c)){perimeter+=1;}

        if c==numcols-1usize {perimeter+=1;}
        else if !comp.contains(&(r,c+1usize)){perimeter+=1;}

    }

    perimeter * (comp.len() as u32)
  }

  fn get_price2(board : & Vec::<Vec::<char>>, numrows : &usize, numcols : &usize, comp : &mut HashSet::<(usize,usize)>)->u32{

    //lets find the perimeter
    //have to do a connected components here too
    let mut up = Vec::<(i32,i32)>::new();
    let mut down = Vec::<(i32,i32)>::new();
    let mut left = Vec::<(i32,i32)>::new();
    let mut right = Vec::<(i32,i32)>::new();

    let mut ch = 'x';

    for vert in comp.iter(){
        let &(r,c) = vert;
        ch = board[r][c];
        if r==0usize {up.push((-1,c as i32));}
        else if board[r-1usize][c] != ch {up.push((r as i32-1,c as i32));}

        if c==0usize {left.push((r as i32,-1 ));}
        else if board[r][c-1usize] != ch {left.push((r as i32,c as i32-1));}

        if r==numrows-1usize {down.push((*numrows as i32,c as i32));}
        else if board[r+1usize][c] != ch {down.push((r as i32+1,c as i32));}

        if c==numcols-1usize {right.push((r as i32, *numcols as i32));}
        else if board[r][c+1usize] != ch {right.push((r as i32,c as i32+1));}

    }

    //ok, now we take sides
    left.sort_by(|a, b| if a.1 != b.1 {return a.1.cmp(&b.1)} else {a.0.cmp(&b.0)});
    right.sort_by(|a, b| if a.1 != b.1 {return a.1.cmp(&b.1)} else {a.0.cmp(&b.0)});
    up.sort_by(|a, b| if a.0 != b.0 {return a.0.cmp(&b.0)} else {a.1.cmp(&b.1)});
    down.sort_by(|a, b| if a.0 != b.0 {return a.0.cmp(&b.0)} else {a.1.cmp(&b.1)});


    //if ch == 'R' {println!(" here is left {:?}",left);}
    //if ch == 'R' {println!(" here is right {:?}",right);}
    //if ch == 'R' {println!(" here is down {:?}",down);}
    //if ch == 'R' {println!(" here is up {:?}",up);}

    let mut numsides = 4u32;

    for ii in 0..(left.len()-1usize) {
        if dist(&left[ii],&left[ii+1]) >1 {numsides +=1;}
    }
    for ii in 0..(right.len()-1usize) {
        if dist(&right[ii],&right[ii+1]) >1 {numsides +=1;}
    }
    for ii in 0..(up.len()-1usize) {
        if dist(&up[ii],&up[ii+1]) >1 {numsides +=1;}
    }
    for ii in 0..(down.len()-1usize) {
        if dist(&down[ii],&down[ii+1]) >1 {numsides +=1;}
    }




    numsides

  }



pub fn part_one(input: &str) -> Option<u32> {
    let mut board : Vec::<Vec::<char>> = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        let row = line.chars().collect();
        board.push(row);
    }

    let numrows = board.len();
    let numcols = board[0].len();

    Some(find_comps(&mut board))

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

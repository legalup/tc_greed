use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(15);


struct bx {
    row : usize,
    lcol : usize,
    rcol : usize
}

impl bx {
    pub fn new(r : usize, lc : usize) -> Self {
        Self {
            row: r,
            lcol : lc,
            rcol : lc+1usize,
        }
    }

}

fn get_nbrs(dirrow : &i32, dircol : &i32, bb : &bx, bxs : &Vec::<bx> )->Vec::<usize>{
    let mut ret = Vec::<usize>::new();
    if *dirrow!=0{
        let nurow = if *dirrow>0 {bb.row + 1usize}
        else {bb.row - 1usize};

        for ii in 0..bxs.len(){
            let nbb = &bxs[ii];
            if nbb.row != nurow {continue;}
            if nbb.lcol == bb.lcol || nbb.lcol==bb.rcol || nbb.rcol==bb.lcol {
                ret.push(ii);
            }
        }
    }
    else {

        let nulcol= if *dircol>0 {bb.rcol+1usize} else {bb.lcol-2usize};

        for ii in 0..bxs.len(){
            let nbb = &bxs[ii];
            if nbb.row == bb.row && nbb.lcol == nulcol  {
                ret.push(ii);
            }
        }
    }

    //get neighbors from nbrs. make hashset so not to have multiplies
    let mut all = HashSet::<usize>::new();
    for idx in ret.iter(){
        all.insert(*idx);
        let nunbrs = get_nbrs(dirrow, dircol, &bxs[*idx], bxs);
        for nunbr in nunbrs.iter(){
            all.insert(*nunbr);
        };
    };

    ret.clear();

    for &hh in all.iter(){
        ret.push(hh);
    };

    ret
}

fn can_move(map : &mut Vec<Vec<char>>, dirrow : &i32, dircol : &i32, bb : &bx, bxs : &Vec::<bx>)->bool{
    let nbrs = get_nbrs(dirrow, dircol, bb, bxs);
    for idx in nbrs.iter(){
        let nbr = &bxs[*idx];
        if !can_move(map, dirrow, dircol, &nbr, bxs) {
            return false;
        }
    }
    if *dirrow!=0{
        let nurow = if *dirrow>0 {bb.row + 1usize}
        else {bb.row - 1usize};
        return  map[nurow][bb.lcol]!='#' && map[nurow][bb.rcol]!='#';
    }
    else {
        if *dircol>0 {return map[bb.row][bb.rcol+1usize]!='#';}
        else {return map[bb.row][bb.lcol-1usize] != '#';};
    }

    true
}

fn moove_bx(dirrow : &i32, dircol : &i32, bb : &mut bx){
    if *dirrow!=0{
        if *dirrow>0 {bb.row +=1usize}
        else {bb.row -= 1usize};

    }
    else {
        if *dircol>0 {bb.lcol+=1usize; bb.rcol+=1usize;}
        else {bb.lcol-=1usize; bb.rcol-=1usize;};
    }
}

fn moove(map : &mut Vec<Vec<char>>,dirrow : &i32, dircol : &i32, bx_idx : &usize , bxs : &mut Vec::<bx>){


    let nbrs = get_nbrs(dirrow, dircol, &bxs[*bx_idx], bxs);
    for idx in nbrs.iter(){
        let nbr = &mut bxs[*idx];
        moove_bx(dirrow, dircol, nbr);
    }

    let bb = &mut bxs[*bx_idx];
    println!("before moove {},{},{}",bb.row,bb.lcol,bb.rcol);
    moove_bx(dirrow, dircol, bb);
    println!("after moove {},{},{}",bb.row,bb.lcol,bb.rcol);


}

fn next(dirrow : &i32, dircol : &i32, map : &mut Vec<Vec<char>>, row : &mut usize, col : &mut usize ){

    let mut stk = VecDeque::<(char, usize, usize)>::new();
    let mut curr_row = row.clone() as i32;
    let mut curr_col = col.clone()  as i32;
    stk.push_back(('@',*row,*col));
    while stk.back().unwrap().0 != '#' && stk.back().unwrap().0 != '.' {
        curr_row += *dirrow;
        curr_col += *dircol;

        stk.push_back((map[curr_row as usize][curr_col as usize],curr_row as usize,curr_col as usize));
    }


    if stk.back().unwrap().0 == '.' {
        //lets unwind this stack, shall we
        let mut poz = stk.pop_back().unwrap();
        let mut write_row = poz.1 as i32;
        let mut write_col = poz.2 as i32;
        while !stk.is_empty(){
            let poz = stk.pop_back().unwrap();
            map[write_row as usize][write_col as usize] = poz.0;
            write_row = poz.1 as i32;
            write_col=  poz.2 as i32;
        }
        map[*row][*col] = '.';
        *row = ((*row as i32)+dirrow) as usize;
        *col = ((*col as i32)+dircol) as usize;

    }



}



pub fn part_one(input: &str) -> Option<u32> {
    let dirs : HashMap<char,(i32,i32)> = HashMap::from([('^', (-1,0)), ('>', (0,1)),('v', (1,0)),('<', (0,-1))]);

    let mut ismap = true;
    let mut commands = Vec::<&str>::new();
    let mut map = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        if line.is_empty(){
            ismap = false;
        }
        else{
            if ismap {
                let row : Vec<char>= line.chars().collect();
                map.push(row);
            }
            else {
                commands.push(line);
            }
        }
    }

    let command = commands.join("");

     println!("here is new map");
    for r in 0..map.len(){
        println!("{:?}",map[r]);
    }
    //println!("here is the command {}",command);
    //first, lets find the location of the robot
    let numrows = map.len();
    let numcols = map[0].len();
    let mut robr = 0usize;
    let mut robc = 0usize;
    for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == '@'{
                robr=r;
                robc = c;
                break;
            }
        }
    }

    for ch in command.chars(){
        next(&dirs[&ch].0, &dirs[&ch].1, &mut map, &mut robr, &mut robc);
    }


    //println!("here is your new map now");
    //for r in 0..map.len(){
    //    println!("{:?}",map[r]);
    //}
    let mut sum = 0u64;
    for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == 'O'{
                let ur = r as u64;
                let uc = c as u64;
                sum += 100u64*ur+uc;
            }
        }
    }

    println!("the final sum is {}", sum);

    Some(0)
}

fn rendermap(robr : usize, robc : usize, width : usize, height : usize, bxs : &Vec::<bx>){
    let mut map = vec![vec!['.';width];height];
    for c in 0..width{
        map[0][c] = '#';
        map[height-1usize][c] = '#';
    }
    for r in 0..height{
        map[r][0] = '#';
        map[r][1] = '#';
        map[r][width-1usize] = '#';
        map[r][width-2usize] = '#';

    }
    for bb in bxs.iter(){
        map[bb.row][bb.lcol] = '[';
        map[bb.row][bb.rcol] = ']';
    }
    map[robr][robc] = '@';
    for r in 0..map.len(){
        println!("{:?}",map[r]);
    }


}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs : HashMap<char,(i32,i32)> = HashMap::from([('^', (-1,0)), ('>', (0,1)),('v', (1,0)),('<', (0,-1))]);

    let mut ismap = true;
    let mut commands = Vec::<&str>::new();
    let mut map = Vec::<Vec::<char>>::new();
    let mut bxs= Vec::<bx>::new();
    for line in input.lines(){
        if line.is_empty(){
            ismap = false;
        }
        else{
            if ismap {
                let row : Vec<char>= line.chars().collect();
                let mut nurow = Vec::<char>::new();
                for ch in row.iter() {
                    match *ch{
                        '.'=> {nurow.push('.');nurow.push('.');},
                        '#'=> {nurow.push('#');nurow.push('#');},
                        'O'=> {nurow.push('[');nurow.push(']');},
                        _=> {nurow.push('@');nurow.push('.');}

                    }
                }
                map.push(nurow);
            }
            else {
                commands.push(line);
            }
        }
    }

    let command = commands.join("");

    //lets update the map now
    //println!("here is the command {}",command);
    //first, lets find the location of the robot
    let numrows = map.len();
    let numcols = map[0].len();
    let mut robr = 0usize;
    let mut robc = 0usize;
    for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == '@'{
                robr=r;
                robc = c;
            }
            else if map[r][c] == '['{
                bxs.push(bx::new(r,c));
            }
        }
    }

    println!("here is your fresh map now. ");
    rendermap(robr, robc, numcols, numrows, &bxs);


    //for ch in command.chars(){
    //    next(&dirs[&ch].0, &dirs[&ch].1, &mut map, &mut robr, &mut robc);
    //}


    //println!("here is your new map now");
    //for r in 0..map.len(){
    //    println!("{:?}",map[r]);
    //}

   /*  for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == 'O'{
                let ur = r as u64;
                let uc = c as u64;
                sum += 100u64*ur+uc;
            }
        }
    } */

    //let testcommnad = "vvvvv";
    //let testrow : Vec<char> = testcommnad.chars().collect();
    for ch in command.chars(){
        let dirrow = &dirs[&ch].0;
        let dircol= &dirs[&ch].1;
        let mut nurow=robr.clone();
        let mut nucol = robc.clone();
        if *dirrow!=0{
            if *dirrow>0 {nurow+=1usize;}
            else {nurow -= 1usize;};
        }
        if *dircol!=0{
            if *dircol>0 {nucol+=1usize;}
            else {nucol -= 1usize;};
        }

        //lets see if there is a box adjoint to the robot. call it bb
        let mut nbr_idx = 0;
        let mut found = false;
        for ii in 0..bxs.len(){
            let bb = &bxs[ii];
            if bb.row==nurow && (bb.lcol == nucol || bb.rcol == nucol){
                nbr_idx = ii;
                found = true;
                break;
            }
        }

        if found {
           // println!("was found!!");
           let bb = & bxs[nbr_idx];

            if can_move(&mut map, dirrow, dircol, bb, &bxs) {
                moove(&mut map, dirrow, dircol, &nbr_idx, &mut bxs);
                robr = nurow;
                robc = nucol;
            }
        }
        else{
            if map[nurow][nucol] != '#' {
            robr = nurow;
            robc = nucol;
            }
        }

    //println!("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");

    //rendermap(robr, robc, numcols, numrows, &bxs);

    }
    println!("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
    rendermap(robr, robc,numcols, numrows, &bxs);

    let mut sum = 0u64;
    for bb in bxs.iter(){
        let fac1 = (bb.lcol as u64) ;
        let fac2 = (bb.row as u64) * 100u64;
        sum += fac1+fac2;
    }



    println!("the final sum for part 2 is {}", sum);

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}

use core::num;
use std::collections::{HashMap, HashSet};


advent_of_code::solution!(6);

//returns false if went off the board
//moves pos or dir. updates board after a move, putting X where it was
fn next(numrows : usize, numcols : usize, board : &mut Vec<Vec<char>>, dir : &mut char, r : &mut i32, c: &mut i32)->bool{

    board[*r as usize][*c as usize] = 'X';

    let oldpos = (*r,*c).clone();
    match *dir {
        '^'=> *r -=1,
        'v'=> *r +=1,
        '<'=> *c -=1,
        _ => *c +=1,
    }
    if *r<0 || *r>=(numrows as i32) || *c<0 || *c >=(numcols as i32) {
        return false;
    }

    if board[*r as usize][*c as usize] == '#' {
        *dir = match *dir{
            '^'=> '>',
            'v'=> '<',
            '<'=> '^',
           _ => 'v',
        };

        (*r,*c) = oldpos;
    }

    true

}

//assume that position and orientation has been record in past BEFORE hitting here
fn next2(numrows : usize, numcols : usize, board : &mut Vec<Vec<char>>, dir : &mut char,
    r : &mut i32, c: &mut i32, past : &mut HashMap<(i32,i32),HashSet<char>>)->i32{



    let oldpos = (*r,*c).clone();
    match *dir {
        '^'=> {*r -=1; }
        'v'=> {*r +=1; }
        '<'=> {*c -=1; }
        _ => {*c +=1; }
    }
    if *r<0 || *r>=(numrows as i32) || *c<0 || *c >=(numcols as i32) {
        board[oldpos.0 as usize][oldpos.1 as usize] = 'X';
        return 1; //indicate you left the board. u r done
    }

    //if new positions is a wall, backtrack, turn right, give correct message
    if board[*r as usize][*c as usize] == '#' {
        *dir = match *dir{
            '^'=> '>',
            'v'=> '<',
            '<'=> '^',
           _ => 'v',
        };

        (*r,*c) = oldpos;
        return 0;
    }
    else{
        //you are moving on board
        //first check already been moved to
        if past.contains_key(&(*r,*c)) {
            if past[&(*r,*c)].contains(dir) {return 2};
            let dircopy = dir.clone();
            past.get_mut(&(*r,*c)).unwrap().insert(dircopy);
        }
        else {
            let dircopy = dir.clone();
            past.insert((*r,*c), HashSet::from([dircopy]));
        }

    }

    0

}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs : HashSet<char> = HashSet::from(['^','v','>','<']);
    let mut board : Vec<Vec<char>> = input.lines().map(|f| f.chars().collect()).collect::<Vec<Vec<char>>>();
    let numrows = board.len();
    let numcols = board[0].len();

    //lets find the position and dir of the guard
    let (mut row, mut col) = (0,0);
    let mut st ='0';

    for r in 0..numrows {
        for c in 0..numcols{
            if dirs.contains(&board[r][c]) {
                //found
                st = board[r][c];
                row = r as i32;
                col = c as i32;
            }
        }
    }


    while next(numrows, numcols, &mut board, &mut st, &mut row, &mut col) {
    }

    //println!("here is board {:?}",board);
    //lets count those Xs
    let mut sum = 0u32;
    for r in 0..numrows {
        for c in 0..numcols{
            if board[r][c]=='X'{
                sum +=1;
            }
        }
    }

    Some(sum)
}

fn see_if_cycles(board : &mut Vec<Vec<char>>, dirs : &HashSet<char>, row : i32, col:i32, st : char)->bool {



    let numrows = board.len();
    let numcols = board[0].len();
    //lets find the position and dir of the guard


    let mut past = HashMap::<(i32,i32),HashSet<char>>::new();
    let mut hs : HashSet<char> = HashSet::from([st]);
    past.insert((row,col),hs);
    let mut r = row.clone();
    let mut c = col.clone();
    let mut dir = st.clone();
    let mut retval = next2(numrows, numcols, board, &mut dir, &mut r, &mut c, &mut past);
    while retval == 0{
        //println!("right before next call");
        retval = next2(numrows, numcols, board, &mut dir, &mut r, &mut c, &mut past);
    }

    if retval == 1 {
        return false;
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs : HashSet<char> = HashSet::from(['^','v','>','<']);
    let mut board : Vec<Vec<char>> = input.lines().map(|f| f.chars().collect()).collect::<Vec<Vec<char>>>();
    let numrows = board.len();
    let numcols = board[0].len();
    let mut dir = '^';
    let mut row = 0;
    let mut col = 0;
    let mut sum = 0u32;

    for r in 0..numrows {
        for c in 0..numcols {
            if board[r][c] == '^' {
                row = r as i32;
                col = c as i32;
            }
        }
    }

    for r in 0..numrows {
        for c in 0..numcols {
            let ch = board[r][c];
            if ch != '#' && !dirs.contains(&ch){
                let mut nuboard = board.clone();
                print!("looking at r {} c {}",r,c);
                //io::stdout().flush();


                nuboard[r][c] = '#';
                if see_if_cycles(&mut nuboard, &dirs, row,col,dir) {
                    sum += 1;
                }
            }
        }
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

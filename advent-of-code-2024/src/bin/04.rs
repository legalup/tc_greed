use std::collections::HashMap;

advent_of_code::solution!(4);

fn find_xmas( board : &Vec<Vec<usize>>, numrows : &usize, numcols : &usize, steprow : i32, stepcol : i32, mut row : &usize, mut col : &usize, step : &usize)->u32 {

    let mut sum = 0u32;
     if *step<3 {
        if board[*row][*col] != *step { return 0u32}
        else {
            //println!("row {}, col {}, step {}",*row,*col,*step);
                    let r = (*row as i32)+steprow;
                    let c = (*col as i32)+stepcol;
                    if r>=0 && c>=0 && (r as usize)<*numrows && (c as usize)<*numcols{
                        sum += find_xmas(board, numrows, numcols, steprow, stepcol, &(r as usize), &(c as usize), &(*step+1));
                    }
        }

     }
     else if *step == 3 && board[*row][*col] == 3 { return 1u32}
     else {
        return 0u32;
     }

    sum



}

pub fn part_one(input: &str) -> Option<u32> {
    let path : HashMap<char,usize> = HashMap::from([('X',0usize),('M',1usize),('A',2usize),('S',3usize)]);
    let board  : Vec<Vec<usize>> = input.lines().map(|f| f.chars().map(|ch| path[&ch]).collect()).collect::<Vec<Vec<usize>>>();

    let nr = board.len();
    let nc = board[0].len();
    let step = 0usize;
    let mut ret = 0u32;
    for r in 0usize..nr{
        for c in 0usize..nc{
            ret += find_xmas(&board, &nr, &nc, -1,0,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, -1,1,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, -1,-1,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, 1,0,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, 1,1,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, 1,-1,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, 0,1,&r, &c, &step);
            ret += find_xmas(&board, &nr, &nc, 0,-1,&r, &c, &step);

        }
    }

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let path : HashMap<char,usize> = HashMap::from([('X',0usize),('M',1usize),('A',2usize),('S',3usize)]);
    let board  : Vec<Vec<usize>> = input.lines().map(|f| f.chars().map(|ch| path[&ch]).collect()).collect::<Vec<Vec<usize>>>();

    let nr = board.len();
    let nc = board[0].len();
    let step = 0usize;
    let mut ret = 0u32;
    for r in 1usize..(nr-1usize){
        for c in 1usize..(nc-1usize){
            if board[r][c] == 2usize {
                let t1 = board[r-1usize][c-1usize]*board[r+1usize][c+1usize];
                let t2 = board[r-1usize][c+1usize]*board[r+1usize][c-1usize];

                if t1 == 3usize && t2==3usize{
                    ret += 1;
                }

            }

        }
    }

    Some(ret)
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

use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Vec::<Vec::<u32>>::new();
    for line in input.lines(){
        let row = line.chars().map(|cc| cc.to_digit(10u32).unwrap()).collect();
        board.push(row);
    }
    let numrows = board.len();
    let numcols = board[0].len();

    let mut trailheads = HashMap::<(usize,usize),HashSet::<(usize,usize)>>::new();

    //initialize our datra struct
    for r in 0..numrows{
        for c in 0..numcols{
            if board[r][c] == 9 {
                trailheads.insert((r,c),HashSet::<(usize,usize)>::from([(r,c)]));
            }
            else{
                trailheads.insert((r,c),HashSet::<(usize,usize)>::new());

            }
        }
    }

    for idx in (0..9).rev(){
        for r in 0..numrows{
            for c in 0..numcols{
                if board[r][c] == idx {
                    if r>0 {
                        if board[r-1][c] == idx+1u32 {
                            let cp = trailheads[&(r-1,c)].clone();
                            trailheads.get_mut(&(r,c)).unwrap().extend(cp.iter());
                        }
                    }
                    if c>0 {
                        if board[r][c-1] == idx+1u32 {
                            let cp = trailheads[&(r,c-1)].clone();
                            trailheads.get_mut(&(r,c)).unwrap().extend(cp.iter());
                        }
                    }
                    if r<numrows-1 {
                        if board[r+1][c] == idx+1u32 {
                            let cp = trailheads[&(r+1,c)].clone();
                            trailheads.get_mut(&(r,c)).unwrap().extend(cp.iter());
                        }
                    }
                    if c<numcols-1 {
                        if board[r][c+1] == idx+1u32 {
                            let cp = trailheads[&(r,c+1)].clone();
                            trailheads.get_mut(&(r,c)).unwrap().extend(cp.iter());
                        }
                    }
                }
            }
        }

    }
    //println!("here is trailheads {:?}",trailheads);

    //now lets get that sum
    let mut sum  = 0u32;
    for r in 0..numrows{
        for c in 0..numcols{
            if board[r][c] == 0 {
                sum += (trailheads[&(r,c)].len() as u32);//.insert((r,c),HashSet::<(usize,usize)>::from([(r,c)]));
            }

        }
    }


    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Vec::<Vec::<u32>>::new();
    for line in input.lines(){
        let row = line.chars().map(|cc| cc.to_digit(10u32).unwrap()).collect();
        board.push(row);
    }
    let numrows = board.len();
    let numcols = board[0].len();

    let mut trailheads = HashMap::<(usize,usize),u32>::new();

    //initialize our datra struct
    for r in 0..numrows{
        for c in 0..numcols{
            if board[r][c] == 9 {
                trailheads.insert((r,c),1u32);
            }
            else{
                trailheads.insert((r,c),0u32);
            }
        }
    }
    for idx in (0..9).rev(){
        for r in 0..numrows{
            for c in 0..numcols{
                if board[r][c] == idx {
                    if r>0 {
                        if board[r-1][c] == idx+1u32 {
                            let cp = trailheads[&(r-1,c)];
                            *trailheads.get_mut(&(r,c)).unwrap() += cp;
                        }
                    }
                    if c>0 {
                        if board[r][c-1] == idx+1u32 {
                            let cp = trailheads[&(r,c-1)];
                            *trailheads.get_mut(&(r,c)).unwrap() += cp;
                        }
                    }
                    if r<numrows-1 {
                        if board[r+1][c] == idx+1u32 {
                            let cp = trailheads[&(r+1,c)];
                            *trailheads.get_mut(&(r,c)).unwrap() += cp;
                        }
                    }
                    if c<numcols-1 {
                        if board[r][c+1] == idx+1u32 {
                            let cp = trailheads[&(r,c+1)];
                            *trailheads.get_mut(&(r,c)).unwrap() += cp;
                        }
                    }
                }
            }
        }

    }

    //now lets get that sum
    let mut sum  = 0u32;
    for r in 0..numrows{
        for c in 0..numcols{
            if board[r][c] == 0 {
                sum += trailheads[&(r,c)];//.insert((r,c),HashSet::<(usize,usize)>::from([(r,c)]));
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

use std::{char, collections::{HashMap, HashSet}, f32::consts::E};

advent_of_code::solution!(8);

fn get_dist(row1 : usize, col1 : usize, row2 : usize, col2 : usize) -> usize {


    let distrow = if row1 < row2 {row2-row1} else {row1-row2};
    let distcol = if col1 < col2 {col2-col1} else {col1-col2};

    distrow + distcol


}

fn is_on_board(row :i32, col:i32, numrows:usize, numcols : usize) ->bool{
    if row<0 || col < 0 {return false};
    if (row as usize)>= numrows {return false;}

    if (col as usize)>= numcols {return false;}
    true

}

fn find_antinodes(row1 : usize, col1 : usize, row2 : usize, col2 : usize, numrows : usize, numcols : usize, antinodes: &mut HashSet<(usize,usize)>){
    let mut dists = HashSet::<usize>::new();
    let dirrow = (row2 as i32)-(row1 as i32);
    let dircol = (col2 as i32)-(col1 as i32);

    let mut nupt = (row1 as i32 + dirrow, col1 as i32+dircol);
    while is_on_board(nupt.0, nupt.1, numrows, numcols){
        let dist1 = get_dist(row1, col1, nupt.0 as usize,nupt.1 as usize);
        let dist2 = get_dist(row2, col2, nupt.0 as usize,nupt.1 as usize);
        if dist1 == 2usize*dist2 || dist2 == 2usize*dist1{
            antinodes.insert((nupt.0 as usize,nupt.1 as usize));
        }
        nupt.0 += dirrow;
        nupt.1 += dircol;
    }

    let mut nupt = (row2 as i32 - dirrow, col2 as i32-dircol);

    while is_on_board(nupt.0, nupt.1, numrows, numcols){
        let dist1 = get_dist(row1, col1, nupt.0 as usize,nupt.1 as usize);
        let dist2 = get_dist(row2, col2, nupt.0 as usize,nupt.1 as usize);

        if dist1 == 2usize*dist2 || dist2 == 2usize*dist1{

            antinodes.insert((nupt.0 as usize,nupt.1 as usize));
        }
        nupt.0 -= dirrow;
        nupt.1 -= dircol;

    }

}

fn find_antinodes2(row1 : usize, col1 : usize, row2 : usize, col2 : usize, numrows : usize, numcols : usize, antinodes: &mut HashSet<(usize,usize)>){
    let mut dists = HashSet::<usize>::new();
    let dirrow = (row2 as i32)-(row1 as i32);
    let dircol = (col2 as i32)-(col1 as i32);

    let mut nupt = (row1 as i32 + dirrow, col1 as i32+dircol);
    while is_on_board(nupt.0, nupt.1, numrows, numcols){
        antinodes.insert((nupt.0 as usize,nupt.1 as usize));
        nupt.0 += dirrow;
        nupt.1 += dircol;
    }

    let mut nupt = (row2 as i32 - dirrow, col2 as i32-dircol);

    while is_on_board(nupt.0, nupt.1, numrows, numcols){
        antinodes.insert((nupt.0 as usize,nupt.1 as usize));
        nupt.0 -= dirrow;
        nupt.1 -= dircol;

    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut nodes = HashMap::<char, Vec<(usize,usize)>>::new();
    let mut board = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        let mut chars : Vec<char> = line.chars().collect();
        board.push(chars);
    }

    let numrows = board.len();
    let numcols = board[0].len();



    for r in 0..numrows{
        for c in 0..numcols{
            let ch = board[r][c];
            if ch != '.' {
                nodes.entry(ch).
                and_modify(|mm| {mm.push((r,c));})
                .or_insert(Vec::<(usize,usize)>::from([(r,c)]));
            }
        }
    }

    //println!("here are nodes {:?}",nodes);


    let mut antinodes = HashSet::<(usize,usize)>::new();
    for vv in nodes.values(){
        let sz = vv.len();
        for ii in 0..sz {
            for jj in (ii+1)..sz{
                find_antinodes(vv[ii].0, vv[ii].1, vv[jj].0, vv[jj].1, numrows, numcols, &mut antinodes);
            }
        }
    }



    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nodes = HashMap::<char, Vec<(usize,usize)>>::new();
    let mut board = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        let mut chars : Vec<char> = line.chars().collect();
        board.push(chars);
    }

    let numrows = board.len();
    let numcols = board[0].len();



    for r in 0..numrows{
        for c in 0..numcols{
            let ch = board[r][c];
            if ch != '.' {
                nodes.entry(ch).
                and_modify(|mm| {mm.push((r,c));})
                .or_insert(Vec::<(usize,usize)>::from([(r,c)]));
            }
        }
    }

    //println!("here are nodes {:?}",nodes);


    let mut antinodes = HashSet::<(usize,usize)>::new();
    for vv in nodes.values(){
        let sz = vv.len();
        for ii in 0..sz {
            for jj in (ii+1)..sz{
                find_antinodes2(vv[ii].0, vv[ii].1, vv[jj].0, vv[jj].1, numrows, numcols, &mut antinodes);
            }
        }
    }



    Some(antinodes.len() as u32)
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

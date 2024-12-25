advent_of_code::solution!(14);

use std::collections::VecDeque;
use std::io::{stdin,stdout};

static WIDTH: i32 = 101;
static HEIGHT: i32 = 103;
static TIME: i32 = 100;

fn make_pos(px : i32, py : i32, vx : i32, vy : i32, pos : &mut Vec::<(i32,i32)>){
    let mut pxest = (px + (TIME*vx)) % WIDTH;
    let mut pyest = (py + (TIME*vy)) % HEIGHT;

    if pxest < 0 {pxest += WIDTH};
    if pyest < 0 {pyest += HEIGHT};


    pos.push((pxest,pyest));


}

fn make_pos2(timmy : &i32, px : i32, py : i32, vx : i32, vy : i32, pos : &mut Vec::<(i32,i32)>){
    let mut pxest = (px + (timmy*vx)) % WIDTH;
    let mut pyest = (py + (timmy*vy)) % HEIGHT;

    if pxest < 0 {pxest += WIDTH};
    if pyest < 0 {pyest += HEIGHT};


    pos.push((pxest,pyest));


}

fn next_pos(pos : &mut VecDeque::<(i32,i32,i32,i32)>){
    let sz = pos.len();
    for ii in 0..sz{
        let (px,py,vx,vy) = pos[ii];
        let mut pxest = (px + vx) % WIDTH;
        let mut pyest = (py + vy) % HEIGHT;

        if pxest < 0 {pxest += WIDTH};
        if pyest < 0 {pyest += HEIGHT};
        pos.push_back((pxest,pyest,vx,vy));

    }

    for ii in 0..sz {
        pos.pop_front();
    }

}



fn display_row(row : &Vec<char>){
    let ss = row.iter().collect::<String>();
    println!("{}",ss);
}
fn display(screen : &Vec<Vec<char>>){
    for row in screen.iter(){display_row(row);}
}

fn disty(x : &i32, y: &i32, mx : &f32, my : &f32 ) ->f64 {
    let fx = *x as f32;
    let fy = *y as f32;
    let dx = (fx-*mx);
    let dy = (fy-*my);
    (dy*dy) as f64
}

fn compute_variance(state : &VecDeque<(i32,i32,i32,i32)>)->f64 {

    //first, let compute mean
    let mut meanx  = 0.0;
    let mut meany = 0.0;

    let sz = state.len();
    for ii in 0..sz{
        meanx += state[ii].0 as f32;
        meany += state[ii].1 as f32;
    }

    meanx = meanx/(sz as f32);
    meany = meany/(sz as f32);

    let mut var = 0f64;
    for ii in 0..sz{
        var += disty(&state[ii].0,&state[ii].1,&meanx, &meany);
    }

    var/(sz as f64)

}

fn display_state(state : &VecDeque<(i32,i32,i32,i32)>)->i32{
    let mut map= vec![vec![0;WIDTH as usize];HEIGHT as usize];
    for &(x,y,vx,vy) in state.iter(){
        map[y as usize][x as usize] += 1;
    }

    let mut screen = vec![vec!['.';WIDTH as usize];HEIGHT as usize];
    for r in 0..HEIGHT {
        for c in 0..WIDTH{
            let ur = r as usize;
            let uc = c as usize;
            screen[ur][uc] = match map[ur][uc] {
                1 => '*',
                2 => '*',
                3 => '*',
                4=>'*',
                _ => '.',
            };
        }
    }

    display(&screen);
    let mut s=String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");


   0

}


pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = Vec::<(i32,i32)>::new();
    for line in input.lines(){
        let toks : Vec<&str> = line.split(['=',' ',',']).into_iter().collect();
        make_pos(toks[1].parse::<i32>().unwrap(),
         toks[2].parse::<i32>().unwrap(),
         toks[4].parse::<i32>().unwrap(),
         toks[5].parse::<i32>().unwrap(),
         &mut pos);
    }

    //println!("here is pos looks like now {:?}",pos);

    let (mut ul,mut ur,mut ll,mut lr) = (0,0,0,0);
    let wb2 = WIDTH/2;
    let hb2 = HEIGHT/2;
    for &(px,py) in pos.iter(){
        if px == wb2 || py==hb2 {continue;}
        if px<wb2{
            if py<hb2 {ul+=1;}
            else {ll+=1;}
        }
        else{
            if py<hb2 {ur+=1;}
            else {lr+=1;}
        }
    }
    println!(" ul {}, ur {}, lr {}, ll {}",ul,ur,lr,ll);
    println!(" the answer is {}", ul*ur*lr*ll);
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = VecDeque::<(i32,i32,i32,i32)>::new();
    for line in input.lines(){
        let toks : Vec<&str> = line.split(['=',' ',',']).into_iter().collect();
        state.push_back((toks[1].parse::<i32>().unwrap(),
        toks[2].parse::<i32>().unwrap(),
        toks[4].parse::<i32>().unwrap(),
        toks[5].parse::<i32>().unwrap()));
    }

    //println!("here is pos looks like now {:?}",pos);
    let mut mxmx = 0;
    let mut best_time = 0;
    let mut best_var = 10000000000.0;
    for timmy in 0..100{
        next_pos(&mut state);
        let vv = compute_variance(&state);
        if vv<best_var{
            best_time = timmy+1;
            best_var = vv;
        }
        //println!("this is time {}",timmy+1);
    }

    println!(" time at lowest var is {} and lowest var is {} ",best_time, best_var);


    Some(0)
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

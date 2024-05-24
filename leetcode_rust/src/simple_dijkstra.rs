/*
You are given an n x n integer matrix board where the cells are labeled from 1 to n2 in a Boustrophedon style starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.

You start on square 1 of the board. In each move, starting from square curr, do the following:

    Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n2)].
        This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
    If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
    The game ends when you reach the square n2.

A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that snake or ladder is board[r][c]. Squares 1 and n2 do not have a snake or ladder.

Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.

    For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.

Return the least number of moves required to reach the square n2. If it is not possible to reach the square, return -1.
*/
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cmp::min;



impl Solution {

    fn state_2_r_c(st : &usize, n: &usize)->(usize,usize) {
        let mst = st-1;

        let mut rem = mst % n;
        let nr =  n-1-(mst-rem)/n;
        let nc = if (n-nr)%2 == 0 {n-rem-1} else {rem};

        (nr,nc)
    }

    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let goal =n*n;

        let mut dists : Vec<_> = (0..=goal).map(|x| i32::MIN).collect();

        let mut heap = BinaryHeap::new();

        let start = (0i32,1usize);
        heap.push(start);

        while let Some((cost,st)) = heap.pop(){

            if cost<=dists[st] {continue;}
            
            //winner, winner, chicken dinner
            dists[st] = cost;
            let (nr,nc) = Self::state_2_r_c(&st,&n);
            let mx = min(st+6,n*n);

            let nexts : Vec<usize> = (st+1..=mx).map(|x| {
                let (nr,nc) = Self::state_2_r_c(&x,&n);
                if board[nr][nc]==-1 {x}else{board[nr][nc] as usize}}).collect();
            for next in nexts{
                heap.push((cost-1,next));
            }
            
        }

        //panic!("wtf dude?");
        if dists[goal] == i32::MIN {return -1} else {-dists[goal]}
    }
}
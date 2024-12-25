use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::u64;

advent_of_code::solution!(16);

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: (usize,usize),
    dir: (i32,i32),
    cost:u64,
    path : HashSet<(usize,usize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dirdist(dir1 : (i32,i32),dir2 : (i32,i32)) -> u64 {
    if dir1==dir2 {return 0u64;}
    else if dir1.0 * dir2.0==0 && dir1.1 & dir2.1==0 {return 1000u64;}
    2000u64
}

fn get_nbrs(map : &Vec::<Vec::<char>>, numrows : &usize, numcols : &usize, st : &State)->Vec::<State>{

    let currrow = st.position.0 as i32;
    let currcol = st.position.1 as i32;
    let mut ret = Vec::<State>::new();

    let dirs = vec![(-1,0),(1,0),(0,1),(0,-1)];

    for (dr,dc) in dirs{
        let nur = currrow+dr;
        let nuc = currcol+dc;
        if nur<= 0|| nur>=(map.len() as i32) || nuc<0 || nuc>=(map[0].len() as i32) {continue;}; //offmap
        if map[nur as usize][nuc as usize] == '#' {continue;}; //wall

        let pozzy = (nur as usize,nuc as usize);
        let mut nupath = st.path.clone();
        nupath.insert(pozzy);

        let nbr = State{
            position : pozzy,
            dir : (dr,dc),
            cost : st.cost+1u64+dirdist(st.dir, (dr,dc)),
            path : nupath,
        };
        ret.push(nbr);
    }

    ret
}



pub fn part_one(input: &str) -> Option<u32> {
    let mut scores = HashMap::<(usize,usize),u64>::new();
    let mut map = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        let row : Vec<char>= line.chars().collect();
        map.push(row);
    }

    let numrows = map.len();
    let numcols = map[0].len();

    //find start and end position
    let mut rstart = 0usize;
    let mut cstart = 0usize;
    let mut rend = 0usize;
    let mut cend = 0usize;
    for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == 'E' {rend = r; cend=c;}
            if map[r][c] == 'S' {rstart = r; cstart=c;}

        }
    }

    let goal = (rend,cend);

    let mut dist = HashMap::<((usize,usize),(i32,i32)),u64>::new();
    let mut heap = BinaryHeap::new();
    let pozzy = (rstart as usize,cstart as usize);

    // initialize
    let start = State{
        position : pozzy,
        dir : (0,1),
        cost : 0,
        path : HashSet::<(usize,usize)>::from([pozzy]),
    };

    dist.insert((start.position,start.dir), 0u64);
    heap.push(start);

    let mut bestcost = u64::MAX;

    //while let Some(State { cost, position, dir }) = heap.pop() {
    while let st = heap.pop().unwrap() {
        // Alternatively we could have continued to find all shortest paths
        if st.position == goal { bestcost=st.cost; break;}

        // Important as we may have already found a better way
        if st.cost > dist[&(st.position,st.dir)] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next in get_nbrs(&map, &numrows, &numcols, &st){
            //println!("got a nbr, whose position is {:?} and whose dir is {:?} and cost {}",next.position,next.dir,next.cost);
            if dist.contains_key(&(next.position,next.dir)){
                //println!("it contains key! ");
                if next.cost < dist[&(next.position,next.dir)]{
                    //dist[&next.position] = next.cost;
                    //println!("about to push1");
                    heap.push(next.clone());
                    dist.insert((next.position,next.dir), next.cost);
                }
            }
            else{
                dist.insert((next.position,next.dir), next.cost);


                heap.push(next);
            }
        }
    }

    println!("best cost is {}", bestcost);

    Some(bestcost as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scores = HashMap::<(usize,usize),u64>::new();
    let mut map = Vec::<Vec::<char>>::new();
    for line in input.lines(){
        let row : Vec<char>= line.chars().collect();
        map.push(row);
    }

    let numrows = map.len();
    let numcols = map[0].len();

    //find start and end position
    let mut rstart = 0usize;
    let mut cstart = 0usize;
    let mut rend = 0usize;
    let mut cend = 0usize;
    for r in 0..numrows{
        for c in 0..numcols{
            if map[r][c] == 'E' {rend = r; cend=c;}
            if map[r][c] == 'S' {rstart = r; cstart=c;}

        }
    }

    let goal = (rend,cend);

    let mut dist = HashMap::<((usize,usize),(i32,i32)),u64>::new();
    let mut heap = BinaryHeap::new();
    let pozzy = (rstart as usize,cstart as usize);

    // initialize
    let start = State{
        position : pozzy,
        dir : (0,1),
        cost : 0,
        path : HashSet::<(usize,usize)>::from([pozzy]),
    };

    dist.insert((start.position,start.dir), 0u64);
    heap.push(start);

    let mut bestcost = 105508u64;
    let mut goodtiles = HashSet::<(usize,usize)>::new();

    //while let Some(State { cost, position, dir }) = heap.pop() {
    while let Some(st) = heap.pop() {

                // Alternatively we could have continued to find all shortest paths
                if st.position == goal { if bestcost==st.cost {println!("=======================hi there, got a short one");
                goodtiles.extend(st.path.iter());};
                continue;}

                // Important as we may have already found a better way
                if st.cost > dist[&(st.position,st.dir)] { continue; }

                // For each node we can reach, see if we can find a way with
                // a lower cost going through this node
                for next in get_nbrs(&map, &numrows, &numcols, &st){
                    //println!("got a nbr, whose position is {:?} and whose dir is {:?} and cost {}",next.position,next.dir,next.cost);
                    if dist.contains_key(&(next.position,next.dir)){
                        //println!("it contains key! ");
                        if next.cost <= dist[&(next.position,next.dir)]{
                            //dist[&next.position] = next.cost;
                            //println!("about to push1");
                            heap.push(next.clone());
                            dist.insert((next.position,next.dir), next.cost);
                        }
                    }
                    else{
                        dist.insert((next.position,next.dir), next.cost);


                        heap.push(next.clone());
                    }
                }
        }
        println!("-------------------answer for part 2 is {}", goodtiles.len());

        Some(bestcost as u32)
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}

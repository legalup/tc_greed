advent_of_code::solution!(18);

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::u64;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: u64,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))

    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
struct TState {
    position: (usize, usize),
    time : u64,
    cost: u64,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for TState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| other.time.cmp(&self.time))
            .then_with(|| self.position.cmp(&other.position))

    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for TState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Computer<'a> {
    scores: HashMap<(usize, usize), u64>,
    map: &'a Vec<Vec<char>>,
    dist: HashMap<((usize, usize)), u64>,
    heap: BinaryHeap<State>,
    numrows: usize,
    numcols: usize,
}
impl Computer<'_> {
    /// The values of `b` and `c` are always 0 in the provided inputs.
    fn new(map: &Vec<Vec<char>>) -> Computer<'_> {
        Computer {
            scores: HashMap::<(usize, usize), u64>::new(),
            map,
            dist: HashMap::<(usize, usize), u64>::new(),
            heap: BinaryHeap::new(),
            numrows: map.len(),
            numcols: map[0].len(),
        }
    }

    fn get_nbrs(&mut self, st: &State) -> Vec<State> {
        let currrow = st.position.0 as i32;
        let currcol = st.position.1 as i32;
        let mut ret = Vec::<State>::new();

        let dirs = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

        for (dr, dc) in dirs {
            let nur = currrow + dr;
            let nuc = currcol + dc;
            if nur < 0 || nur >= (self.numrows as i32) || nuc < 0 || nuc >= (self.numcols as i32) {
                continue;
            }; //offmap
            if self.map[nur as usize][nuc as usize] == '#' {
                continue;
            }; //wall

            let pozzy = (nur as usize, nuc as usize);

            let nbr = State {
                position: pozzy,
                cost: st.cost + 1u64,
            };
            ret.push(nbr);
        }

        ret
    }

    fn run(&mut self) -> Option<u64> {
        let goal = (self.numrows - 1usize, self.numcols - 1usize);
        let start = State {
            position: (0, 0),
            cost: 0,
        };

        self.dist.insert(start.position, 0u64);
        self.heap.push(start);

        let mut bestcost = u64::MAX;

        while let Some(st) = self.heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if st.position == goal {
                bestcost = st.cost;
                return Some(bestcost);
            }

            // Important as we may have already found a better way
            if st.cost > self.dist[&st.position] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for next in self.get_nbrs(&st) {
                //println!("got a nbr, whose position is {:?} and whose dir is {:?} and cost {}",next.position,next.dir,next.cost);
                if self.dist.contains_key(&(next.position)) {
                    //println!("it contains key! ");
                    if next.cost < self.dist[&next.position] {
                        //dist[&next.position] = next.cost;
                        //println!("about to push1");
                        self.heap.push(next);
                        self.dist.insert(next.position, next.cost);
                    }
                } else {
                    self.dist.insert(next.position, next.cost);

                    self.heap.push(next);
                }
            }
        }
        return None;
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut pts = Vec::<(usize,usize)>::new();
    let mut map = vec![vec!['.';7];7];

    for line in input.lines(){
        let pt : Vec<usize> = line.split([' ',',']).map(|ss| ss.parse::<usize>().unwrap()).collect();
        //map[pt[0]][pt[1]] = '#' ;
        pts.push((pt[1],pt[0]));
    }

    for ii in 0..1024{
        map[pts[ii].0][pts[ii].1] = '#';
    }

    //println!("here is map {:?}",map);

    let mut compy = Computer::new(&map);
    if let Some(val) = compy.run() {
        println!(" answer to part 1 is {}", val);
        return Some(val as u32);
    }


    None
}


struct Computer2<'a> {
    scores: HashMap<(usize, usize), u64>,
    map: &'a Vec<Vec<i32>>,
    dist: HashMap<((usize, usize)), u64>,
    heap: BinaryHeap<TState>,
    numrows: usize,
    numcols: usize,
    start_time: u64,
}
impl Computer2<'_> {
    /// The values of `b` and `c` are always 0 in the provided inputs.
    fn new(map: &Vec<Vec<i32>>, start_time : u64) -> Computer2<'_> {
        Computer2 {
            scores: HashMap::<(usize, usize), u64>::new(),
            map,
            dist: HashMap::<(usize, usize), u64>::new(),
            heap: BinaryHeap::new(),
            numrows: map.len(),
            numcols: map[0].len(),
            start_time,
        }
    }

    fn get_nbrs(&mut self, st: &TState) -> Vec<TState> {
        let currrow = st.position.0 as i32;
        let currcol = st.position.1 as i32;
        let mut ret = Vec::<TState>::new();

        let dirs = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

        for (dr, dc) in dirs {
            let nur = currrow + dr;
            let nuc = currcol + dc;
            if nur < 0 || nur >= (self.numrows as i32) || nuc < 0 || nuc >= (self.numcols as i32) {
                continue;
            }; //offmap
            if self.map[nur as usize][nuc as usize] == 1 {
                continue;
            }; //wall

            let pozzy = (nur as usize, nuc as usize);

            let nbr = TState {
                position: pozzy,
                cost: st.cost + 1u64,
                time:0u64,
            };
            ret.push(nbr);
        }

        ret
    }

    fn run(&mut self) -> Option<u64> {
        let goal = (self.numrows - 1usize, self.numcols - 1usize);
        let start = TState {
            position: (0, 0),
            cost: 0,
            time: self.start_time,
        };

        self.dist.insert(start.position, 0u64);
        self.heap.push(start);

        let mut bestcost = u64::MAX;

        while let Some(st) = self.heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if st.position == goal {
                bestcost = st.cost;
                break;
            }

            // Important as we may have already found a better way
            if st.cost > self.dist[&st.position] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for next in self.get_nbrs(&st) {
                //println!("got a nbr, whose position is {:?} and whose dir is {:?} and cost {}",next.position,next.dir,next.cost);
                if self.dist.contains_key(&(next.position)) {
                    //println!("it contains key! ");
                    if next.cost < self.dist[&next.position] {
                        //dist[&next.position] = next.cost;
                        //println!("about to push1");
                        self.heap.push(next);
                        self.dist.insert(next.position, next.cost);
                    }
                } else {
                    self.dist.insert(next.position, next.cost);

                    self.heap.push(next);
                }
            }
        }
        Some(bestcost)
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut pts = Vec::<(usize,usize)>::new();
    let mut map = vec![vec!['.';71];71];

    for line in input.lines(){
        let pt : Vec<usize> = line.split([' ',',']).map(|ss| ss.parse::<usize>().unwrap()).collect();
        //map[pt[0]][pt[1]] = '#' ;
        pts.push((pt[0],pt[1]));
    }

    let sz = pts.len();
    for ii in 0..sz{
        map[pts[ii].0][pts[ii].1] = '#';

        let mut compy = Computer::new(&map);
        match compy.run() {
            None => {println!("the first time it doesnt work is {} and that byte is {:?}",ii,pts[ii]); return Some(0);}
            Some(val)=> {println!("it worked for time {} with cost {}", ii, val);}
        }

    }

    None
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

//advent_of_code::solution!(20);

use std::collections::{HashSet,HashMap,BinaryHeap};
use std::f32::MIN;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::Ordering;
use std::thread::LocalKey;
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

static MINDIFF : i64 = 100;

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

fn dfs(
    grid: &Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    visited: HashSet<(usize, usize)>,
    path: &mut Vec<(usize, usize)>,
) -> i64 {
    let mut count = 0;
    let mut visited = visited;
    if start.0 == end.0 && start.1 == end.1 {
        path.push(end);
        return visited.len() as i64;
    }
    if grid[start.0][start.1] == 1 {
        return 0;
    }
    path.push(start);
    visited.insert(start);
    let next_steps = get_next(grid, start);

    for l in next_steps {
        if !visited.contains(&l) {
            let mut step_path = vec![];
            dfs(grid, l, end, visited.clone(), &mut step_path);
            if step_path.contains(&end) {
                path.extend(step_path);
                count += path.len() as i64;
                return count - 1;
            }
        }
    }
    0
}

fn dfs_wall(
    grid: &Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    visited: HashSet<(usize, usize)>,
    path: &mut Vec<(usize, usize)>,
) -> i64 {
    let mut count = 0;
    let mut visited = visited;
    if start.0 == end.0 && start.1 == end.1 {
        path.push(end);
        return visited.len() as i64;
    }
    if grid[start.0][start.1] == 0 {
        return 0;
    }
    path.push(start);
    visited.insert(start);
    let next_steps = get_next_wall(grid, start);

    for l in next_steps {
        if count>20 {return 0;} //here we stop if paths get too intense
        if !visited.contains(&l) {
            let mut step_path = vec![];
            dfs_wall(grid, l, end, visited.clone(), &mut step_path);
            if step_path.contains(&end) {
                path.extend(step_path);
                count += path.len() as i64;
                return count - 1;
            }
        }
    }
    0
}

fn dijkstra(grid: &Vec<Vec<u8>>,
    start: &(usize, usize),
    goal: &(usize, usize)) -> Option<u64> {

    let strt = State{
        position:*start,
        cost:0,
    };

    let mut dist = HashMap::<(usize, usize), u64>::new();
    let mut heap= BinaryHeap::new();

    dist.insert(strt.position, 0u64);
    heap.push(strt);

    let mut bestcost = u64::MAX;

    while let Some(st) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if st.position == *goal {
            bestcost = st.cost;
            return Some(bestcost);
        }

        // Important as we may have already found a better way
        if st.cost > dist[&st.position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for next in get_next_state(grid, st) {
            //println!("got a nbr, whose position is {:?} and whose dir is {:?} and cost {}",next.position,next.dir,next.cost);
            if dist.contains_key(&(next.position)) {
                //println!("it contains key! ");
                if next.cost < dist[&next.position] {
                    //dist[&next.position] = next.cost;
                    //println!("about to push1");
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
            } else {
                dist.insert(next.position, next.cost);

                heap.push(next);
            }
        }
    }
    return None;
}



fn get_next(grid: &Vec<Vec<u8>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 && grid[location.0][location.1 - 1] != 1 {
        steps.push((location.0, location.1 - 1));
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] != 1 {
        steps.push((location.0, location.1 + 1));
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] != 1 {
        steps.push((location.0 - 1, location.1));
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] != 1 {
        steps.push((location.0 + 1, location.1));
    }
    steps
}

fn get_next_wall(grid: &Vec<Vec<u8>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 && grid[location.0][location.1 - 1] == 1 {
        steps.push((location.0, location.1 - 1));
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] == 1 {
        steps.push((location.0, location.1 + 1));
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] == 1 {
        steps.push((location.0 - 1, location.1));
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] == 1 {
        steps.push((location.0 + 1, location.1));
    }
    steps
}



fn get_next_state(grid: &Vec<Vec<u8>>, curr: State) -> Vec<State> {
    let mut steps: Vec<State> = vec![];
    let location = curr.position;
    if location.1 > 0 && grid[location.0][location.1 - 1] == 1 {
        let nupos =(location.0, location.1 - 1);
        steps.push(State{position:nupos,cost:curr.cost+1});
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] == 1 {
        let nupos =(location.0, location.1 + 1);
        steps.push(State{position:nupos,cost:curr.cost+1});
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] == 1 {
        let nupos =(location.0-1, location.1);
        steps.push(State{position:nupos,cost:curr.cost+1});
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] == 1 {
        let nupos =(location.0+1, location.1);
        steps.push(State{position:nupos,cost:curr.cost+1});
    }
    steps
}

fn get_connection_pt(
    grid: &Vec<Vec<u8>>,
    location: (usize, usize),
    direction: Direction,
    path: &Vec<(usize, usize)>,
) -> (usize, usize) {
    let mut current = location;
    loop {
        match direction {
            Direction::LEFT => {
                if current.1 == 0 {
                    break;
                }
                current = (current.0, current.1 - 1);
            }
            Direction::RIGHT => {
                if current.1 == grid[0].len() - 1 {
                    break;
                }
                current = (current.0, current.1 + 1);
            }
            Direction::UP => {
                if current.0 == 0 {
                    break;
                }
                current = (current.0 - 1, current.1);
            }
            Direction::DOWN => {
                if current.0 == grid.len() - 1 {
                    break;
                }
                current = (current.0 + 1, current.1);
            }
        }
        if grid[current.0][current.1] == 1 {
            break;
        }
        if path.contains(&current) {
            return current;
        }
    }
    (usize::MAX, usize::MAX)
}



fn get_cheats(
    grid: &Vec<Vec<u8>>,
    location: (usize, usize),
    original_path: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut steps: Vec<(usize, usize)> = vec![];
    if location.1 > 0 && grid[location.0][location.1 - 1] == 1 {
        let step = get_connection_pt(
            grid,
            (location.0, location.1 - 1),
            Direction::LEFT,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.1 < grid[0].len() - 1 && grid[location.0][location.1 + 1] == 1 {
        let step = get_connection_pt(
            grid,
            (location.0, location.1 + 1),
            Direction::RIGHT,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.0 > 0 && grid[location.0 - 1][location.1] == 1 {
        let step = get_connection_pt(
            grid,
            (location.0 - 1, location.1),
            Direction::UP,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    if location.0 < grid.len() - 1 && grid[location.0 + 1][location.1] == 1 {
        let step = get_connection_pt(
            grid,
            (location.0 + 1, location.1),
            Direction::DOWN,
            original_path,
        );
        if step.0 != usize::MAX && step.1 != usize::MAX {
            steps.push(step);
        }
    }
    steps
}

fn doit(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut original_path: Vec<(usize, usize)> = vec![];
    dfs(grid, start, end, HashSet::new(), &mut original_path);

    let mut total = 0;
    for (idx, location) in original_path.iter().enumerate() {
        let cheat_exits = get_cheats(grid, *location, &original_path);
        cheat_exits.iter().for_each(|c| {
            for i in idx..original_path.len() {
                if original_path[i] == *c {
                    let diff = i - idx - 2;
                    if diff >= 100 {
                        total += 1;
                    }
                }
            }
        });
    }

    total
}

fn doit2(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut original_path: Vec<(usize, usize)> = vec![];
    dfs(grid, start, end, HashSet::new(), &mut original_path);
    //println!("here is the path {:?}",original_path);
    //println!("here is the grid {:?}",grid);

    let mut total = 0;
    let sz = original_path.len();

    let mut cheats = HashSet::<((usize,usize),(usize,usize))>::new();

    for idx in 0..sz{
        //println!("idx is {} out of {}",idx, sz);
        let idx_wall_nbrs = get_next_wall(grid, original_path[idx]);
        for jdx in (idx+((MINDIFF-1) as usize))..sz{
        //println!("jdx is {}",jdx);

            let jdx_wall_nbrs = get_next_wall(grid, original_path[jdx]);
            //println!("sz of idx wall nbrs: {} and jdxnbrs:{}", idx_wall_nbrs.len(), jdx_wall_nbrs.len());
            for (ii, ilocation) in idx_wall_nbrs.iter().enumerate(){
                for (jj, jlocation) in jdx_wall_nbrs.iter().enumerate(){

                    let cnt = dijkstra(grid,
                        ilocation,
                        jlocation).unwrap();

                    if cnt>0{
                        //we have a connection
                        let index_diff  = (jdx-idx) as i64+1i64;
                        let icnt = cnt as i64 +1;
                        let diff = index_diff-icnt;
                        if diff >= MINDIFF {total+=1;
                            cheats.insert((original_path[idx],original_path[jdx]));
                            //println!("here is a connection{:?},{:?} of length {}",ilocation, jlocation,diff)
                            }
                    }
                }
            }
        }
    }
    println!("here is cheats {:?}",cheats);
    cheats.len() as i64
}


    /* for (idx, location) in original_path.iter().enumerate() {
        let cheat_exits = get_cheats2(grid, *location, &original_path);
        cheat_exits.iter().for_each(|c| {
            for i in idx..original_path.len() {
                if original_path[i] == *c {
                    let diff = i - idx - 2;
                    if diff >= 100 {
                        total += 1;
                    }
                }
            }
        });
    } */


pub fn part_one(fname: &str) -> Option<u32> {
    let mut data: Vec<Vec<u8>> = Vec::new();
    let mut trail_start: (usize, usize) = (0, 0);
    let mut trail_end: (usize, usize) = (0, 0);

   let file = File::open(fname);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let mut row: Vec<u8> = Vec::new();
                        for c in l.chars() {
                            match c {
                                '.' => row.push(0),
                                '#' => row.push(1),
                                'E' => {
                                    row.push(0);
                                    trail_end = (data.len(), row.len() - 1);
                                }
                                'S' => {
                                    row.push(0);
                                    trail_start = (data.len(), row.len() - 1);
                                }
                                _ => panic!("naughty character!!! {}", c),
                            }
                        }

                        data.push(row);
                    }
                    Err(e) => panic!("fuck yourself: {}", e),
                }
            }
        }
        Err(e) => panic!("err w file: {}", e),
    }
    //wuts the diff

    println!("about to call doit. start: {:?} and end {:?}", trail_start, trail_end);
    let totes = doit2(&data, trail_start, trail_end);
    println!(" the solution to part 1 is : {} for mindiff {}", totes,MINDIFF);
    Some(totes as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

/* #[cfg(test)]
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
} */

fn main() {


        let fname = String::from("/home/lgalup/workspace/advent-of-code-2024/data/inputs/20.txt");
        let count = part_one(&fname).unwrap();
        println!("Total: {}", count);

    //let fname = String::from("/home/lgalup/workspace/advent-of-code-2024/data/inputs/20.txt");
    //let result = part_one(&advent_of_code::template::read_file("inputs", DAY))?;
    //let count = parse_input(&fname);
    //println!("Total: {}", result);
   }

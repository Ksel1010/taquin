use crate::board::*;
use crate::heuristics;
use crate::heuristics::*;
use crate::min_heap::*;
use Direction::*;
use std::collections::*;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}

impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}
/// for A* use a weight of 1 , for dijkstra use a weight of 0 and for suboptimal A* use a weight > 1
pub fn search(init_state: Board, heuristic: &Heuristic, weight:u32) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut costs: HashMap<Board, u32> = HashMap::new();
    // HashMap to store the expanded  nodes 
    let mut expanded : HashMap<Board, u32> = HashMap::new();
    // HashMap to store the action done to the parent to get the node 
    let mut parent : HashMap<Board, (Board,Direction)> = HashMap::new();
    // Option<Vec<Direction>> to store the actions resolving in the Goal
    let mut plan: Option<Vec<Direction>>  = None;

    heap.insert(init_state, 0);
    costs.insert(init_state, 0);

    while !heap.is_empty(){
        let mut s = heap.pop().unwrap();
        if expanded.contains_key(&s){
            continue;
        }
        if Board::GOAL == s {
            let mut directions: Vec< Direction> = Vec::new();
            while s!=init_state{
                let data: &(Board, Direction) = parent.get(&s).unwrap();
                s = data.0;
                directions.insert(0,data.1);
            }
            plan = Some(directions);
            break;
        }
        for direction in [Right, Left, Up, Down]{
            match s.apply(direction){
                Some(s_prime) => {
                    let c = costs.get(&s).unwrap() + 1 ;

                    if !costs.contains_key(&s_prime) || c < *costs.get(&s_prime).unwrap_or(&std::u32::MAX){

                        costs.insert(s_prime, c);
                        parent.insert(s_prime, (s,direction));
                        heap.insert(s_prime,c+ weight * heuristic.estimate(&s_prime));

                    }
                },
                None=>continue,
            }
        }
        expanded.insert(s, 0);
    }

    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    // example to construct a Stats instance
    let stats = Stats::new(expanded.len(), runtime);
    // return the results and associated stats
    (plan, stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, &Heuristic::Blind, 3);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
            //println!("{init}");
            println!("{0}, {1}", stats.expanded, stats.runtime.as_millis());
            
        }
    }
}

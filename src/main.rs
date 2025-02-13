#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;
use Direction::*;

fn main() {
   /*let initial_board = Board::new([[1, 2, 3],
                                                  [4, 8, 5],
                                                  [0, 7, 6]]
    );
    let plan : &[Direction] = &[Right, Up, Right, Down];
    if initial_board.is_valid_plan(plan) {
        println!("This plan is valid");
        initial_board.play(plan);
     }else{
        panic!("The plan is not valid !!")
     } 
     */

     // validates that search oes return the optimal plan on the first 20 isntances
     for w in 1..4{
      println!("Weight : {w}");
      
      for h in [&Heuristic::Blind, &Heuristic::Hamming, &Heuristic::Manhattan]{
         match h {
            &Heuristic::Blind => println!("Blind: "),
            &Heuristic::Hamming => println!("Hamming: "),
            &Heuristic::Manhattan => println!("Manhatten: "),
         }
         for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init, h, w);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            if path.len()!=*expected_cost as usize{
               println!("Error : path not optimal it costs {0} but the optimal only costs {1}", path.len(), expected_cost);
            }
            //assert_eq!(path.len(), *expected_cost as usize);
            //println!("{init}");
            println!("{0}; {1}", stats.expanded, stats.runtime.as_millis());
        }
        println!("\n\n\n");
      }
   }     
  
}

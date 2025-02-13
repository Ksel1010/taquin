use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut distance: u32 = 0;
                for i in 0..self::N{
                    for j in 0..self::N{
                        if board.value_at(i, j) != Board::GOAL.value_at(i, j){
                            distance+=1;
                        }
                    }
                }
                distance
            }
            
            
            Heuristic::Manhattan => {
                let mut distance = 0;
                for i in 0..self::N{
                    for j in 0..self::N{
                        distance+=  manhattan_peer(board.value_at(i, j), (i.try_into().unwrap(), j.try_into().unwrap())); 
                    }
                }
                distance.try_into().unwrap()
            }
        }
    }
    
}

fn manhattan_peer(cell:u8, (line, column):(u8, u8)) -> u8 {
    let expected:(u8, u8) = match  cell {
        0 => (2,2),
        1 => (0,0),
        2 => (0,1),
        3 => (0,2),
        4 => (1,0),
        5 => (1,1),
        6 => (1,2),
        7 => (2,0),
        8 => (2,1),
        _ => panic!("Out of bands ")
    };
    line.abs_diff(expected.0) + column.abs_diff(expected.1)
}



#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 8);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 16);
    }
}

use std::ops::{Index, IndexMut};
use eframe::egui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Icon {
    Char(char),
    Image(&'static str)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Cell {
    Player(Icon),
    Cpu(Icon), 
    Uninitialised
}

impl Cell {
    pub fn new_player(icon: Icon) -> Self {
        Self::Player((icon))
    }

    pub fn new_cpu(icon: Icon) -> Self {
        Self::Cpu(icon)
    }
}

///Used to handle everything that the tic tac toe grid needs!
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Grid([Cell; 9]);

///Save some logic later on by implementing basic formatting
// #[deprecated(since = "commit 5", note = "new function using egui in progress")]
// impl Display for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (i, row) in self.0.iter().enumerate() {
//             for (j, &cell) in row.iter().enumerate() {
//                 let width = cell.width().unwrap_or(1);
//                 let padding = 3 - width;
//                 write!(f, " {}{} ", cell, " ".repeat(padding))?;
//                 if j < 2 {
//                     write!(f, "|")?;
//                 }
//             }
//             if i < 2 {
//                 writeln!(f, "\n-----+-----+-----")?;
//             }
//         }
//         Ok(())
//     }
// }

///Save some .0 calls to get around the tuple
impl Index<usize> for Grid {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        if index > 9 {
            panic!("Attempted to index the Grid immutably with an out-of-bounds index");
        }
        &self.0[index]
    }
}

///Save some .0 calls to get around the tuple, this time for modifying the grid
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 9 {
            panic!("Attempted to index the Grid mutably with an out-of-bounds index");
        }
        &mut self.0[index]
    }
}

///Easier creation
impl Default for Grid {
    fn default() -> Self {
        Self([Cell::Uninitialised; 9])
    }
}

///Used to handle the possible grid states
pub(crate) enum States {
    Win(Icon),
    Draw,
    Ongoing
}

impl Grid {
    const WINNING_POSITIONS: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];
    ///Check for wins, draws, or ongoing
    pub(crate) fn get_state(&self) -> States {
        let grid = &self.0;
        //If the grid signature matches any winning position, return a Win, along with the winner
        for &pos in Self::WINNING_POSITIONS.iter() {
            if grid[pos[0]] == grid[pos[1]] && 
                grid[pos[1]] == grid[pos[2]] {
                return match grid[pos[0]] {
                    Cell::Cpu(icon)|Cell::Player(icon) => States::Win(icon),
                    Cell::Uninitialised => unreachable!()
                }
            }
        }
        //If there is an unselected cell, the game is still ongoing!
        for &cell in grid {
            if let Cell::Uninitialised = cell {
                return States::Ongoing;
            }
        }
        //The grid is full, return a Draw
        States::Draw
    }
}


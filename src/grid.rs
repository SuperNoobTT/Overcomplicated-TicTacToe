use std::{fmt::Display, ops::{Index, IndexMut}};
use unicode_width::UnicodeWidthChar;

///Used to handle everything that the tic tac toe grid needs!
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Grid([[char; 3]; 3]);

///Save some logic later on by implementing basic formatting
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let width = cell.width().unwrap_or(1);
                let padding = 3 - width;
                write!(f, " {}{} ", cell, " ".repeat(padding))?;
                if j < 2 {
                    write!(f, "|")?;
                }
            }
            if i < 2 {
                writeln!(f, "\n-----+-----+-----")?;
            }
        }
        Ok(())
    }
}

///Save some .0 calls to get around the tuple
impl Index<usize> for Grid {
    type Output = [char; 3];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

///Save some .0 calls to get around the tuple, this time for modifying the grid
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

///Easier creation
impl Default for Grid {
    fn default() -> Self {
        Self([['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']])
    }
}

///Used to handle the possible grid states
pub(crate) enum States {
    Win(char),
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
        //Create a 1 by 9 representation of the grid for easier state checking
        let mut flattened: [char; 9] = ['a'; 9];
        let mut i: usize = 0;
        for &row in self.0.iter() {
            for &cell in row.iter() {
                flattened[i] = cell;
                i += 1;
            }
        }
        //If the grid signature matches any winning position, return a Win, along with the winner
        for &pos in Self::WINNING_POSITIONS.iter() {
            if flattened[pos[0]] == flattened[pos[1]] && 
                flattened[pos[1]] == flattened[pos[2]] {
                return States::Win(flattened[pos[0]]);
            }
        }
        //If there is an unselected cell, the game is still ongoing!
        for cell in flattened {
            if cell.is_numeric() {
                return States::Ongoing;
            }
        }
        //The grid is full, return a Draw
        States::Draw
    }
}


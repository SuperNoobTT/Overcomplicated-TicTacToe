use std::{array, ops::{Index, IndexMut}};

#[derive(Clone, Copy, Hash, Eq, Debug)]
pub(crate) enum Icon {
    Char(char),
    Image(&'static str)
}

impl PartialEq for Icon {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Icon::Char(chr), Icon::Char(other_chr)) => chr == other_chr,
            (Icon::Image(img), Icon::Image(other_img)) => img == other_img,
            _ => false
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, Debug)]
pub(crate) enum Cell {
    Player(Icon),
    Cpu(Icon), 
    Uninitialised(usize)
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cell::Cpu(icon), Cell::Cpu(other_icon)) => icon == other_icon,
            (Cell::Player(icon), Cell::Player(other_icon)) => icon == other_icon,
            (Cell::Uninitialised(idx), Cell::Uninitialised(other_idx)) => idx == other_idx,
            _ => false
        }
    }
}

///Used to handle everything that the tic tac toe grid needs!
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub(crate) struct Grid([Cell; 9]);

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
        Self(array::from_fn(|i| Cell::Uninitialised(i)))
    }
}

///Used to handle the possible grid states
#[derive(Clone, Hash, Eq, PartialEq, Debug, Default)]
pub(crate) enum GridStates {
    Win(Icon),
    Draw,
    #[default]
    Ongoing
}

impl Grid {
    pub(crate) fn iter<'a>(&'a self) -> core::slice::Iter<'a, Cell>{
        self.0.iter()
    }

    pub(crate) fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, Cell> {
        self.0.iter_mut()
    }

    const WINNING_POSITIONS: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];
    ///Check for wins, draws, or ongoing
    //TODO: Perhaps could be optimised by only checking lines which include the changed cell (take idx as parameter)
    pub(crate) fn get_state(&self) -> GridStates {
        let grid = &self.0;
        //If the grid signature matches any winning position, return a Win, along with the winner
        for &pos in Self::WINNING_POSITIONS.iter() {
            if grid[pos[0]] == grid[pos[1]] && 
                grid[pos[1]] == grid[pos[2]] {
                return match grid[pos[0]] {
                    Cell::Cpu(icon)|Cell::Player(icon) => GridStates::Win(icon),
                    Cell::Uninitialised(_) => unreachable!()
                };
            }
        }
        //If there is an unselected cell, the game is still ongoing!
        for cell in grid.iter() {
            if let Cell::Uninitialised(_) = cell {
                return GridStates::Ongoing;
            }
        }
        //The grid is full, return a Draw
        GridStates::Draw
    }
}


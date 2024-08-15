use crate::grid::{Grid, GridStates, Cell, Icon};
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Board {
    pub grid: Grid,
    pub player_icon: Icon,
    pub cpu_icon: Icon,
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            grid: Grid::default(), 
            player_icon: Icon::Char('❌'),
            cpu_icon: Icon::Char('⭕')
        }
    }
}

impl Board {
    pub fn cpu_turn(&mut self) -> usize {
        //Find the 'best' move (according to our algorithm) and update the grid accordingly
        let best_move = self.find_best_move();
        self.grid[best_move] = Cell::Cpu(self.cpu_icon);
        //Return the chosen box to be outputted for clarity
        best_move
    }

    ///Super suboptimal algorithm, but I'm very lazy :P
    fn find_best_move(&self) -> usize {
        // Check for immediate win
        if let Some(winning_move) = self.find_winning_move(Cell::Cpu(self.cpu_icon)) {
            return winning_move;
        }

        // Block opponent's winning move
        if let Some(blocking_move) = self.find_winning_move(Cell::Player(self.player_icon)) {
            return blocking_move;
        }

        // If center is free, take it
        if let Cell::Uninitialised(_) = self.grid[4] {
            return 4;
        }

        // Prefer corners
        const CORNERS: [usize; 4] = [0, 2, 6, 8];
        for &corner in CORNERS.iter() {
            if let Cell::Uninitialised(_) = self.grid[corner] {
                return corner;
            }
        }

        // Take any available space (only edges left so can skip even indexes)
        (1..8).step_by(2)
            .find(|&n| {
            let curr_cell: Cell = self.grid[n];
            matches!(curr_cell, Cell::Uninitialised(_))
            })
            .expect("There should always be at least one empty area")
    }

    fn find_winning_move(&self, player: Cell) -> Option<usize> {
        for i in 0..9 {
            if let Cell::Uninitialised(_) = self.grid[i] {
                //Clone bc I'm too lazy to find an in-place check :>
                let mut future_grid: Grid = self.grid.clone();
                future_grid[i] = player;
                //If there's an immediate win for either player, return the coords!
                if let GridStates::Win(_) = future_grid.get_state() {
                    return Some(i);
                }
            }
        }
        //If no immediate win, return nothing :B
        None
    }
}

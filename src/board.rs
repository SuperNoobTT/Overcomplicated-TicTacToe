use crate::{grid::{Grid, States, Cell, Icon}, utils::input_helper};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Turns {
    Player,
    Cpu
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Board {
    pub(crate) grid: Grid,
    pub(crate) turn: Turns,
    player_icon: Icon,
    cpu_icon: Icon,
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            grid: Grid::default(), 
            turn: Turns::Player,
            player_icon: Icon::Char('❌'),
            cpu_icon: Icon::Char('⭕')
        }
    }
}

impl Board {
    pub(crate) fn start(&mut self) -> GameResult {
        //Start the game with a new turn!
        self.new_turn()
    }

    #[deprecated(since = "Started move to EGUI")]
    pub(crate) fn custom_icon(&mut self) {
        let user_fn = |input: String| {
            ["player", "cpu"].contains(&input.to_lowercase().as_str())
            .then(|| input.to_lowercase())
            .ok_or("Invalid user !!!")
        };
        // loop {
        //     // println!("The current icons are: \nPlayer: {}, \nCPU: {}", self.player_icon, self.cpu_icon);

        //     let input = input_helper("Would you like to customise the game icons? (Enter Y/N)", |input: String| {
        //         match input.to_lowercase().as_str() {
        //             "y"|"n" => Ok(input.to_lowercase()),
        //             _ => Err("Unrecognised input!")
        //         }
        //     });

        //     if input == "n".to_string() {
        //         break
        //     } else {
        //         assert!(input == "y".to_string()); //This should always be true bc of closure check
        //         //Get the new unicode icon
        //         let icon: char = input_helper("Please enter the icon you would like to use (a single unicode character)", |icon: char| {
        //             if icon.is_numeric() {
        //                 Err("Digits are reserved for the game, please choose a non-digit unicode character!")
        //             } else {
        //                 Ok(icon)
        //             }
        //         });
        //         //Get the user who will be represented by the icon
        //         let user: String = input_helper("Please select whether the icon should be for the cpu or player", user_fn);
        //         //Match the user and update icons appropriately
        //         if user == "player".to_string() {
        //             self.player_icon = grid::Icon::Char(icon);
        //         } else {
        //             assert!(user == "cpu".to_lowercase().to_string()); //Should never fail
        //             self.cpu_icon = icon;
        //         };
        //     }
        // }
    }

    fn new_turn(&mut self) -> GameResult {
        // println!("Current grid: \n{:}", &self.grid); **Deprecated!**
        //Match on the current turn and run the appropriate code
        match self.turn {
            Turns::Player => self.player_turn(),
            Turns::Cpu => self.cpu_turn()
        }
        //Check for the current game state: Win, Draw, or Ongoing
        match self.grid.get_state() {
            States::Draw => { //Game ends in a draw!
                println!("The game ends in a Draw!");
                return GameResult::Draw;
            },
            States::Win(icon) => { //Someone wins!
                //Match on the icons and return the winner to update the game

                // println!("{icon} wins! \n"); **deprecated**
                if icon == self.player_icon {
                    println!("You WIN :> \n");
                    return GameResult::PlayerWin;
                } else {
                    assert_eq!(icon, self.cpu_icon); // This is always the cpu_icon 
                    // println!("The final board: \n{:} \n", self.grid); **commented to remove error lol
                    println!("The CPU Wins :P \n");
                    return GameResult::CPUWin;
                }
            },
            //The game is still ongoing, continue into the next turn!
            States::Ongoing => self.new_turn()
        }
        
    }

    fn player_turn(&mut self) {
        let choice_fn = |input: usize| {
            if input > 9 || input < 1 {
                return Err("Invalid digit!");
            }
            let input = input - 1;
            if let Cell::Uninitialised = self.grid[input] {
                return Ok(input);
            } 
            Err("Selected an already selected box!")
        };
        //Handle the user's input and grab the indexes to update the grid
        let i = input_helper("Please choose an unselected box and enter its digit", choice_fn);
        self.grid[i] = Cell::new_player(self.player_icon);
        //It's now the cpu's turn
        self.turn = Turns::Cpu;
    }

    fn cpu_turn(&mut self) {
        //Find the 'best' move (according to our algorithm) and update the grid accordingly
        let best_move = self.find_best_move();
        self.grid[best_move] = Cell::new_cpu(self.cpu_icon);
        self.turn = Turns::Player;
        //Output the chosen box for clarity
        println!("The cpu chose box number: {} \n", best_move+1);
    }

    ///Super suboptimal algorithm, but I'm very lazy :P
    fn find_best_move(&self) -> usize {
        // Check for immediate win
        if let Some(winning_move) = self.find_winning_move(Cell::new_cpu(self.cpu_icon)) {
            return winning_move;
        }

        // Block opponent's winning move
        if let Some(blocking_move) = self.find_winning_move(Cell::new_player(self.player_icon)) {
            return blocking_move;
        }

        // If center is free, take it
        if let Cell::Uninitialised = self.grid[4] {
            return 4;
        }

        // Prefer corners
        const CORNERS: [usize; 4] = [0, 2, 6, 8];
        for &corner in CORNERS.iter() {
            if let Cell::Uninitialised = self.grid[corner] {
                return corner;
            }
        }

        // Take any available space (only edges left so can skip even indexes)
        (1..8).step_by(2)
            .find(|&n| {
            let curr_cell: Cell = self.grid[n];
            matches!(Cell::Uninitialised, curr_cell)
            })
            .expect("There should always be at least one empty area")
    }

    fn find_winning_move(&self, player: Cell) -> Option<usize> {
        for i in 0..9 {
            if let Cell::Uninitialised = self.grid[i] {
                //Clone bc I'm too lazy to find an in-place check :>
                let mut future_grid: Grid = self.grid.clone();
                future_grid[i] = player;
                //If there's an immediate win for either player, return the coords!
                if let States::Win(_) = future_grid.get_state() {
                    return Some(i);
                }
            }
        }
        //If no immediate win, return nothing :B
        None
    }
}

///Enum to keep track of possible results
pub(crate) enum GameResult {
    PlayerWin, 
    CPUWin,
    Draw
}

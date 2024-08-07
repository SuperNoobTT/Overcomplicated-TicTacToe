use std::{io::{stdin, Write}, str::FromStr};
use crate::grid::{Grid, States};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Turns {
    Player,
    Cpu
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Board {
    pub(crate) grid: Grid,
    pub(crate) turn: Turns,
    player_icon: char,
    cpu_icon: char,
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            grid: Grid::default(), 
            turn: Turns::Player,
            player_icon: '❌',
            cpu_icon: '⭕'
        }
    }
}

impl Board {
    pub(crate) fn start(&mut self) -> GameResult {
        let user_fn = |input: String| {
            ["player", "cpu"].contains(&input.to_lowercase().as_str())
            .then(|| input.to_lowercase())
            .ok_or("Invalid user !!!")
        };
        loop {
            println!("The current icons are: \nPlayer: {}, \nCPU: {}", self.player_icon, self.cpu_icon);

            let input = input_helper("Would you like to customise the game icons? (Enter Y/N)", |input: String| {
                match input.to_lowercase().as_str() {
                    "y"|"n" => Ok(input.to_lowercase()),
                    _ => Err("Unrecognised input!")
                }
            });

            if input == "n".to_string() {
                break
            } else {
                assert!(input == "y".to_string()); //This should always be true bc of closure check
                //Get the new unicode icon
                let icon: char = input_helper("Please enter the icon you would like to use (a single unicode character)", |icon: char| {Ok(icon)});
                //Get the user who will be represented by the icon
                let user: String = input_helper("Please select whether the icon should be for the cpu or player", user_fn);
                //Match the user and update icons appropriately
                if user == "player".to_string() {
                    self.player_icon = icon;
                } else {
                    assert!(user == "cpu".to_lowercase().to_string()); //Should never fail
                    self.cpu_icon = icon;
                };
            }
        }
        //Start the game with a new turn!
        self.new_turn()
    }

    fn new_turn(&mut self) -> GameResult {
        println!("Current grid: \n{:}", &self.grid); //Print the current grid
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
                println!("{icon} wins! \n");
                if icon == self.player_icon {
                    println!("You WIN :> \n");
                    return GameResult::PlayerWin;
                } else {
                    assert_eq!(icon, self.cpu_icon); // This is always the cpu_icon 
                    println!("The final board: \n{:} \n", self.grid);
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
            let (i, j) = (input/3, input%3);
            if self.grid[i][j].is_numeric() {
                return Ok((i, j));
            } 
            Err("Selected an already selected box!")
        };
        //Handle the user's input and grab the indexes to update the grid
        let (i, j): (usize, usize) = input_helper("Please choose an unselected box and enter its digit", choice_fn);
        self.grid[i][j] = self.player_icon;
        //It's now the cpu's turn
        self.turn = Turns::Cpu;
    }

    fn cpu_turn(&mut self) {
        //Find the 'best' move (according to our algorithm) and update the grid accordingly
        let best_move = self.find_best_move();
        let (i, j) = best_move;
        self.grid[i][j] = self.cpu_icon;
        self.turn = Turns::Player;
        //Output the chosen box for clarity
        println!("The cpu chose box number: {} \n", (best_move.0*3 + best_move.1) + 1);
    }

    ///Super suboptimal algorithm, but I'm very lazy :P
    fn find_best_move(&self) -> (usize, usize) {
        // Check for immediate win
        if let Some(winning_move) = self.find_winning_move(self.cpu_icon) {
            return winning_move;
        }

        // Block opponent's winning move
        if let Some(blocking_move) = self.find_winning_move(self.player_icon) {
            return blocking_move;
        }

        // If center is free, take it
        if self.grid[1][1].is_numeric() {
            return (1, 1);
        }

        // Prefer corners
        const CORNERS: [usize; 4] = [0, 2, 6, 8];
        for &corner in CORNERS.iter() {
            let (i, j) = (corner / 3, corner % 3);
            if self.grid[i][j].is_numeric() {
                return (i, j);
            }
        }

        // Take any available space (only edges left so can skip even indexes)
        let idx: usize = (1..8).step_by(2)
            .find(|&n| {
            let (i, j) = (n / 3, n % 3);
            self.grid[i][j].is_numeric()
            })
            .expect("There should always be at least one empty area");
        (idx/3, idx%3)
    }

    fn find_winning_move(&self, player: char) -> Option<(usize, usize)> {
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[i][j].is_numeric() {
                    //Clone bc I'm too lazy to find an in-place check :>
                    let mut future_grid: Grid = self.grid.clone();
                    future_grid[i][j] = player;
                    //If there's an immediate win for either player, return the coords!
                    if let States::Win(_) = future_grid.get_state() {
                        return Some((i, j));
                    }
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

///What
/// Lol this is pretty poorly written but it works :L
/// Used to get inputs, do some logic, and print error statements!
pub(crate) fn input_helper<IN, OUT, F>(request: &str, mut logic: F) -> OUT 
    where 
        F: FnMut(IN) -> Result<OUT, &'static str>,
        IN: FromStr
    {
        loop {
            let mut input: String = String::new();
            println!("{}", request);
            std::io::stdout().flush().expect("Failed to flush :thonk:");
            stdin().read_line(&mut input).expect("Failed to read line, maybe too long?");
            let input = input.trim_end();
            if let Ok(num) = input.parse() {
                match logic(num) {
                    Ok(result) => return result,
                    Err(msg) => eprintln!("{msg}")
                }
            } else {
                eprintln!("Could not parse your input, please ensure you've entered a valid input");
            }
        }
    }
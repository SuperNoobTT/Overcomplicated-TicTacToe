use rand::prelude::*;
use std::{fmt::Display, io::{stdin, Write}, ops::{Index, IndexMut}, str::FromStr};
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone, PartialEq)]
struct Grid([[char; 3]; 3]);

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

impl Index<usize> for Grid {
    type Output = [char; 3];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self([['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']])
    }
}

enum States {
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
    fn get_state(&self) -> States {
        let mut flattened: [char; 9] = ['a'; 9];
        let mut i: usize = 0;
        for &row in self.0.iter() {
            for &cell in row.iter() {
                flattened[i] = cell;
                i += 1;
            }
        }
        for &pos in Self::WINNING_POSITIONS.iter() {
            if flattened[pos[0]] == flattened[pos[1]] && 
                flattened[pos[1]] == flattened[pos[2]] {
                return States::Win(flattened[pos[0]]);
            }
        }
        for cell in flattened {
            if cell.is_numeric() {
                return States::Ongoing;
            }
        }
        States::Draw
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Turns {
    Player,
    Cpu (usize, usize)
}

impl Turns {
    fn player() -> Self {
        Self::Player
    }

    fn cpu() -> Self {
        let mut rng = thread_rng();
        Self::Cpu(rng.gen_range(0..3), rng.gen_range(0..3))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    grid: Grid,
    turn: Turns,
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
    fn start(&mut self) -> GameResult {
        let user_fn = |input: String| {
            if ["player", "cpu"].contains(&input.to_lowercase().as_str()) {
                return Ok(input.to_lowercase());
            }
            Err("Invalid user !!!")
        };
        loop {
            println!("The current icons are: \nPlayer: {}, \nCPU: {}", self.player_icon, self.cpu_icon);
            let input = Self::input_helper("Would you like to customise the game icons? (Enter Y/N)", |input: String| Ok(input));
            if input == "Y" {
                let _ = Self::input_helper("Please enter the icon you would like to use (unicode character)", |icon: char| {
                    let user: String = Self::input_helper("Please select whether the icon should be for the cpu or player", user_fn);
                    if user == "player".to_string() {
                        self.player_icon = icon;
                    } else {
                        self.cpu_icon = icon;
                    }
                    Ok::<&str, &str>("Succesfuly Completed!")
                });
            } else if input == "N" {
                break;
            } else {
                println!("unrecognised input");
            }
        }
        self.new_turn()
    }

    fn new_turn(&mut self) -> GameResult {
        println!("Current grid: \n{:}", &self.grid);
        match self.turn {
            Turns::Player => self.player_turn(),
            Turns::Cpu(i, j) => self.cpu_turn(i, j)
        }
        match self.grid.get_state() {
            States::Draw => {
                println!("The game ends in a Draw!");
                return GameResult::Draw;
            },
            States::Win(icon) => {
                println!("{icon} wins!");
                if icon == self.player_icon {
                    println!("You WIN :>");
                    return GameResult::PlayerWin;
                } else {
                    assert_eq!(icon, self.cpu_icon); // This is always the cpu_icon 
                    println!("The CPU Wins :P");
                    return GameResult::CPUWin;
                }
            },
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
        let (i, j): (usize, usize) = Self::input_helper("Please choose an unselected box and enter its digit", choice_fn);
        self.grid[i][j] = self.player_icon;
        loop {
            let turn: Turns = Turns::cpu();
            if let Turns::Cpu(i, j) = turn {
                if self.grid[i][j].is_numeric() {
                    self.turn = turn;
                    break;
                }
            }
        }

    }

    fn input_helper<IN, OUT, F>(request: &str, mut logic: F) -> OUT 
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
            }
        }
    }

    fn cpu_turn(&mut self, i: usize, j: usize) {
        self.grid[i][j] = self.cpu_icon;
        self.turn = Turns::player();
        println!("The cpu chose box number: {}", i*3 + j+1);
    }
}

enum GameResult {
    PlayerWin, 
    CPUWin,
    Draw
}

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Game {
    pub wins: u32, 
    pub losses: u32,
    pub draws: u32,
    board: Board
}

impl Game {
    fn new_game(&mut self) {
        self.board.grid = Default::default();

        let result: GameResult = self.board.start();
        match result {
            GameResult::CPUWin => self.losses += 1,
            GameResult::Draw => self.draws += 1,
            GameResult::PlayerWin => self.wins += 1
        }
    }
    pub fn play(&mut self) {
        println!("Your current record is: \nWins: {} \nLosses: {} \n Draws: {}", &self.wins, &self.losses, &self.draws);
        let new_game: bool = Board::input_helper("Would you like to start a new game? (Y/N)", |input: String| {
            if input == "Y".to_string() {
                return Ok(true);
            } else if input == "N".to_string() {
                return Ok(false);
            } else {
                return Err("Unrecognised input!");
            }
        });
        let start: String = Board::input_helper("Would you like to start first? (Y/N)", |input: String| {
            if ["Y", "N"].contains(&input.as_str()) {Ok(input)} else {Err("Unrecognised input!")}}
        );
        self.board.grid = Default::default();
        if start == "Y".to_string() {
            self.board.turn = Turns::player();
        } else {
            assert_eq!(&start, &"N".to_string()); //Only two cases bc of closure
            self.board.turn = Turns::cpu();
        }
        if new_game {
            self.new_game();
            self.play();
        }
    }
}


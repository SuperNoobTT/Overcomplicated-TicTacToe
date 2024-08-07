mod board;
pub(crate) mod grid;
use board::{Board, GameResult, Turns, input_helper};

///Literally what it says lol, this handles the game in its entirety
#[derive(PartialEq, Clone, Debug, Default)]
pub struct Game {
    pub wins: u32, 
    pub losses: u32,
    pub draws: u32,
    board: Board
}

impl Game {
    ///Start a new game, and update the scores based on the result of the game
    fn new_game(&mut self) {
        self.board.grid = Default::default();

        let result: GameResult = self.board.start();
        match result {
            GameResult::CPUWin => self.losses += 1,
            GameResult::Draw => self.draws += 1,
            GameResult::PlayerWin => self.wins += 1
        }
    }
    ///The first function called, handles 
    pub fn play(&mut self) {
        println!("Your current record is: \nWins: {} \nLosses: {} \nDraws: {}", &self.wins, &self.losses, &self.draws);
        let new_game: bool = input_helper("Would you like to start a new game? (Y/N)", |input: String| {
            let input: String = input.to_lowercase();
            if input.as_str() == "y" {
                return Ok(true);
            } 
            if input.as_str() == "n" {
                return Ok(false);
            }
            Err("Unrecognised input!")
        });
        
        if new_game {
            let start: String = input_helper("Would you like to start first? (Y/N)", |input: String| {
                match input.to_lowercase().as_str() {
                    "y"|"n" => Ok(input.to_lowercase()),
                    _ => Err("Unrecognised input!")
                }
            });
            self.board.grid = Default::default();
            if start == "y".to_string() {
                self.board.turn = Turns::Player;
            } else {
                assert_eq!(&start, &"n".to_string()); //Only two cases bc of closure
                self.board.turn = Turns::Cpu;
            }
            self.new_game();
            self.play();
        }
    }
}

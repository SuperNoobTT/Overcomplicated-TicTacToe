mod board;
pub(crate) mod display;
pub(crate) mod grid;
use board::Board;
use display::DisplayPlugin;
use bevy::prelude::*;

///Literally what it says lol, this handles the game in its entirety
#[derive(PartialEq, Clone, Debug, Default, Resource)]
pub(crate) struct Game {
    pub wins: u32, 
    pub losses: u32,
    pub draws: u32,
    board: Board
}

    ///The first function called, handles resetting the grid and choosing the starting player
pub fn play() {
    //Allow user to customise player icons before the game starts
    //Todo: Fill this in lol

    App::new()
        .init_resource::<Game>()
        .add_plugins(DisplayPlugin)
        .run();
        

}



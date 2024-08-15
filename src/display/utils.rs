use bevy::prelude::*;
use bevy_egui::egui::{popup_below_widget, Id, PopupCloseBehavior, Response, Ui};
use crate::{Game, grid::GridStates, display::{AppState, game_logic::GameResult}};

#[derive(Default, Event, PartialEq, Clone)]
pub struct PlayerInput {
    pub idx: usize
}

impl PlayerInput {
    pub fn new(idx: usize) -> Self {
        Self{idx}
    }
}

pub fn create_popup(ui: &mut Ui, response: Response, text: &'static str) {
    let popup_id = Id::new("popup_id");

    if response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    popup_below_widget(
        ui,
        popup_id,
        &response,
        PopupCloseBehavior::CloseOnClickOutside,
        |ui| {
            ui.set_min_width(300.0);
            ui.label(text.to_string());
        },
    );
}

pub fn check_win(
    game: &mut ResMut<Game>,
    next_state: &mut ResMut<NextState<AppState>>,
    result: &mut ResMut<GameResult>
) {
    let game_state: GridStates = game.board.grid.get_state();
    if ! matches!(game_state, GridStates::Ongoing) {
        result.0 = game_state;
        next_state.set(AppState::Finish)
    }
}
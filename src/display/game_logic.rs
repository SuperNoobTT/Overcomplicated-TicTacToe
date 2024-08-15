use bevy::prelude::*;
use bevy_egui::{egui::{self, InnerResponse}, EguiContexts};
use crate::{
    display::{NewGame, Turn, PADDING, AppState, utils::{PlayerInput, check_win}},
    Game,
    grid::{Cell, GridStates}
};


#[derive(Resource, Clone, Hash, Eq, PartialEq, Debug, Default)]
struct Settings {
    starting_player: Turn
}

#[derive(Resource, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub(crate) struct GameResult(pub GridStates);

#[derive(SystemSet, Clone, Hash, Eq, PartialEq, Debug, Default)]
struct InGame;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Settings>()
            .init_resource::<GameResult>()
            .configure_sets(Update, InGame.run_if(in_state(AppState::GridScreen)))
            .add_systems(Update, accept_user_input.run_if(in_state(Turn::Player)).in_set(InGame))
            .add_systems(Update, cpu_turn.run_if(in_state(Turn::Cpu)).in_set(InGame))
            .add_systems(Update, new_game.run_if(in_state(AppState::Setup)))
            .add_systems(Update, game_finished.run_if(in_state(AppState::Finish)));
    }
}

fn accept_user_input(
    mut input: EventReader<PlayerInput>,
    mut next_turn: ResMut<NextState<Turn>>,
    mut game: ResMut<Game>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_result: ResMut<GameResult>
) {
    let mut inputs = input.read();
    assert!(inputs.len() < 2); // We only allow 1 input at a time?
    while let Some(player_input) = inputs.next() {
        game.board.grid[player_input.idx] = Cell::Player(game.board.player_icon);
        check_win(&mut game, &mut app_state, &mut game_result);
        next_turn.set(Turn::Cpu);
    }
}

fn cpu_turn(
    mut game: ResMut<Game>,
    mut ctx: EguiContexts,
    mut next_turn: ResMut<NextState<Turn>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_result: ResMut<GameResult>
) {
    let changed_cell: usize = game.board.cpu_turn();

    check_win(&mut game, &mut app_state, &mut game_result);
    next_turn.set(Turn::Player);
}

fn new_game(
    new_game_ev: EventReader<NewGame>, //TODO: Allow this to handle mods and stuff
    game: ResMut<Game>, 
    mut ctx: EguiContexts,
    mut settings: ResMut<Settings>,
    mut next_turn: ResMut<NextState<Turn>>,
    mut app_state: ResMut<NextState<AppState>>,
) {

    egui::Window::new("Mods")
        .collapsible(false)
        .resizable(false)
        .show(ctx.ctx_mut(), |ui| {
            ui.set_min_width(300.0);
            egui::ComboBox::from_label("Select the starting player")
                .selected_text(format!("{}", settings.starting_player))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.starting_player, Turn::Cpu, "Cpu");
                    ui.selectable_value(&mut settings.starting_player, Turn::Player, "Player")
                });
            
            ui.add_space(PADDING);
            if ui.button("Close").clicked() {
                next_turn.set(settings.starting_player);
                app_state.set(AppState::GridScreen)
            }
        });
}

fn game_finished(
    game_result: Res<GameResult>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game: ResMut<Game>,
    mut ctx: EguiContexts
) {
    match game_result.0 {
        GridStates::Win(icon) => {
            let text: &'static str = {
                if icon == game.board.player_icon {
                    game.wins += 1;
                    "You are the winner!"
                } else if icon == game.board.cpu_icon {
                    game.losses += 1;
                    "The Computer wins :P"
                } else {
                    unreachable!() //If some other icon wins, something has gone very wrong lol
                }
            };

            let response = egui::Window::new("Winner")
                .collapsible(false)
                .resizable(false)
                .show(ctx.ctx_mut(), |ui| {
                    ui.label(text);
                });

            if let Some(InnerResponse{response: click, ..}) = response {
                if click.clicked_elsewhere() {
                    app_state.set(AppState::StartScreen);
                }
            } 
        },
        GridStates::Draw => {
            let response = egui::Window::new("Draw")
                .collapsible(false)
                .resizable(false)
                .show(ctx.ctx_mut(), |ui| {
                    ui.label("The game has ended in a draw!");
                });

            if let Some(InnerResponse{response: click, ..}) = response {
                if click.clicked_elsewhere() {
                    app_state.set(AppState::StartScreen);
                }
            } 
        },
        _ => unreachable!() //We should only change to this stage if either a draw or win has happened
    } 
}
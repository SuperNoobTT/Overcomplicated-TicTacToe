use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
mod game_logic;
mod utils;
use game_logic::LogicPlugin;
use utils::{PlayerInput, create_popup};
use crate::{grid::{Cell, Icon, GridStates}, Game};

const PADDING: f32 = 20.0;

#[derive(Event, PartialEq)]
pub(crate) struct NewGame;

#[derive(Resource, Clone, Copy, PartialEq, Default)]
struct PopupState {
    show_mods_popup: bool
}

pub(crate) struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultPlugins, EguiPlugin, grid_plugin, start_plugin, LogicPlugin))
            .init_state::<AppState>()
            .init_state::<Turn>()
            .add_event::<NewGame>()
            .add_event::<PlayerInput>()
            .init_resource::<PopupState>()
            .add_systems(Startup, setup);  
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum Turn {
    #[default]
    Player,
    Cpu
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::Player => write!(f, "Player"),
            Turn::Cpu => write!(f, "Cpu")
        }
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    StartScreen,
    Setup,
    GridScreen,
    Finish
}

fn setup(mut ctx: EguiContexts) {
    ctx.ctx_mut().style_mut(|style| {
        //FIXME: impl dynamic sizing lol
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(36.0, egui::FontFamily::Proportional),
        ); //Make the buttons font a little bigger :B (Pray this doesn't backfire lol)

        let mut visuals = style.visuals.clone();

        // Background color
        visuals.window_fill = egui::Color32::from_rgba_unmultiplied(25, 25, 35, 255); // Dark blue-ish base
        visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(35, 35, 50, 255); // Slightly lighter for panels

        // Text colors
        visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 220)); // Light grey for better contrast

        // Widget colors
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_unmultiplied(45, 45, 65, 255);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(55, 55, 75, 255);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(65, 65, 90, 255);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(75, 75, 105, 255);

        // Accent color
        visuals.selection.bg_fill = egui::Color32::from_rgb(100, 100, 220); // Bluish highlight

        // Rounded corners
        visuals.window_rounding = 8.0.into();
        visuals.widgets.noninteractive.rounding = 5.0.into();
        visuals.widgets.inactive.rounding = 5.0.into();
        visuals.widgets.hovered.rounding = 5.0.into();
        visuals.widgets.active.rounding = 5.0.into();

        style.visuals = visuals;
    });
}

fn grid_plugin(app: &mut App) {
    app
        .add_systems(Update, grid_screen.run_if(in_state(AppState::GridScreen)));
}

fn start_plugin(app: &mut App) {
    app
        .add_systems(Update, start_screen.run_if(in_state(AppState::StartScreen)))
        .add_systems(OnExit(AppState::StartScreen), 
            |mut new_game_ev: EventWriter<NewGame>, mut popup_state: ResMut<PopupState>| {
            new_game_ev.send(NewGame);
        });
}

fn start_screen(
    mut ctx: EguiContexts,
    mut next_state: ResMut<NextState<AppState>>,
    mut popup_state: ResMut<PopupState>
) {
    egui::TopBottomPanel::top("title_panel").show(ctx.ctx_mut(), |ui| {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| {
            ui.heading("My Awesome Game");
        });
        ui.add_space(PADDING);
    });

    egui::SidePanel::right("button_panel")
        .resizable(false)
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Start").clicked() {
                    next_state.set(AppState::Setup);
                }

                ui.add_space(PADDING);
                if ui.button("Load Game").clicked() {
                    ui.label("Unimplemented!");
                }

                ui.add_space(PADDING);
                if ui.button("Mods").clicked() {
                    popup_state.show_mods_popup = !popup_state.show_mods_popup;
                }

                ui.add_space(PADDING);
            });
            ui.add_space(ui.available_height() - 3.0 * ui.spacing().interact_size.y - 2.0 * PADDING);
        });

    // Central area for the popup
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        if popup_state.show_mods_popup {
            display_mods_popup(ui, popup_state)
        }
    });
}

fn display_mods_popup(ui: &mut egui::Ui, mut popup_state: ResMut<PopupState>) {
    //FIXME: This dosn't work!
    //TODO: Add a mods resource that holds the enabled state of all the mods and pass it to the settings resource
    egui::Window::new("Mods")
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            ui.set_min_width(300.0);
            ui.label("Mod Options:");
            if ui.checkbox(&mut false, "Amnesia").enabled() {
                // Implement amnesia mod logic (some cells randomly switch)
            }
            ui.add_space(PADDING);
            if ui.checkbox(&mut false, "3D Grid").enabled() {
                // Implement 3D grid mod logic
            }
            // Add more mod options as needed
            ui.add_space(PADDING);
            if ui.button("Close").clicked() {
                popup_state.show_mods_popup = false;
            }
        });
}

fn grid_screen(
    mut ctx: EguiContexts, 
    mut next_state: ResMut<NextState<AppState>>,
    curr_turn: Res<State<Turn>>,
    game: Res<Game>,
    input_ev: EventWriter<PlayerInput>
) {
    egui::SidePanel::right("Exit/Save").show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            if ui.button("Save current progress").clicked {
                //TODO: Add feature to save the current game struct, e.g. with serde-json
            }
            if ui.button("Exit to main menu").clicked {
                next_state.set(AppState::StartScreen);
            }
        });
    });

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let available_size = ui.available_size();
        let cell_size = (available_size.x.min(available_size.y) - 4.0 * PADDING) / 3.0;

        ui.add_space(PADDING);
        
        create_grid(ui, cell_size, game, input_ev, curr_turn);
    });
}

fn create_grid(
    ui: &mut egui::Ui, cell_size: f32, 
    game: Res<Game>,
    mut input_ev: EventWriter<PlayerInput>,
    curr_turn: Res<State<Turn>>
) {
    const SCALING: f32 = 0.8;
    let mut cells = game.board.grid.iter();
    egui::Grid::new("Board").num_columns(3).spacing([PADDING, PADDING]).show(ui, |ui| {
        for _row in 0..3 {
            for _col in 0..3 {
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(cell_size, cell_size),
                    egui::Sense::click(),
                );

                if ui.is_rect_visible(rect) {
                    let visuals = ui.style().interact(&response);

                    ui.painter().rect(
                        rect,
                        0.0,
                        visuals.bg_fill,
                        visuals.bg_stroke,
                    );

                    let cell = cells.next();

                    //TODO: Match the boards icon enum and use text display for a char, img display for an image
                    match cell {
                        Some(Cell::Player(Icon::Char(chr)) | Cell::Cpu(Icon::Char(chr))) => {
                            let galley = ui.painter().layout_no_wrap(
                                chr.to_string(),
                                egui::FontId::proportional(cell_size * SCALING), // Adjust this factor to change text size
                                visuals.text_color(),
                            );
        
                            let text_pos = rect.center() - galley.size() / 2.0;
                            ui.painter().galley(text_pos, galley, egui::Color32::LIGHT_BLUE);
                        },
                        Some(Cell::Player(Icon::Image(img)) | Cell::Cpu(Icon::Image(img))) => {
                            let image = egui::Image::new(format!("file://{img}"))
                                .fit_to_exact_size(egui::vec2(cell_size * SCALING, cell_size * SCALING));
                            image.paint_at(ui, rect);
                        },
                        Some(Cell::Uninitialised(idx)) => {
                            let galley = ui.painter().layout_no_wrap(
                                idx.to_string(),
                                egui::FontId::proportional(cell_size * SCALING), // Adjust this factor to change text size
                                visuals.text_color(),
                            );
        
                            let text_pos = rect.center() - galley.size() / 2.0;
                            ui.painter().galley(text_pos, galley, egui::Color32::LIGHT_BLUE);
                        },
                        None => unreachable!()
                    }

                    if response.clicked() {
                        if let Turn::Cpu = curr_turn.get() {
                            create_popup(ui, response, "It is not your turn!");
                            continue;
                        }

                        if let Some(Cell::Uninitialised(idx)) = cell {
                            input_ev.send(PlayerInput::new(*idx));
                        } else {
                            create_popup(ui, response, "You cannot change an already occupied cell!");
                        }
                    }
                }
            }
            ui.end_row();
        }
    });
}


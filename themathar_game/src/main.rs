mod config;
mod types;
mod board;
mod heartbeat;
#[path = "ui/mod.rs"]
mod ui_module;

use crate::types::*;
use crate::ui_module as ui;
use crate::heartbeat::HeartbeatState;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

fn main() {
    // This is only called in native builds
    #[cfg(not(target_arch = "wasm32"))]
    run_game();
}

#[wasm_bindgen]
pub fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".to_string()),
                prevent_default_event_handling: false,
                resolution: (1920, 1080).into(),  // Full HD for hotseat
                ..default()
            }),
            ..default()
        }))
            .init_state::<GameState>()
            .init_resource::<Board>()
            .init_resource::<Lobby>()
            .init_resource::<GameSession>()
            .init_resource::<ReplaySystem>()
            .init_resource::<ReplayBoard>()
            .init_resource::<LocalPlayerSlot>()
            .init_resource::<HeartbeatState>()
            .init_resource::<PlayerName>()
            .init_resource::<ui::player_setup::PlayerSetupState>()
            .add_systems(Startup, spawn_camera)
            // Menu state
            .add_systems(OnEnter(GameState::Menu), ui::menu::setup_menu)
            .add_systems(Update, (
                ui::menu::menu_input,
                ui::menu::credits_button_input,
            ).run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), ui::cleanup::cleanup_ui)
            // Credits state
            .add_systems(OnEnter(GameState::Credits), ui::credits::setup_credits)
            .add_systems(Update, ui::credits::credits_input.run_if(in_state(GameState::Credits)))
            .add_systems(OnExit(GameState::Credits), ui::cleanup::cleanup_ui)
            // Player setup - enter player names for hotseat multiplayer
            .add_systems(OnEnter(GameState::PlayerSetup), ui::player_setup::setup_player_setup)
            .add_systems(Update, ui::player_setup::handle_player_setup.run_if(in_state(GameState::PlayerSetup)))
            .add_systems(OnExit(GameState::PlayerSetup), ui::cleanup::cleanup_ui)
            // Playing
            .add_systems(OnEnter(GameState::Playing), ui::game::setup_game)
            .add_systems(Update, (
                ui::game::handle_mask_activation,
                ui::game::handle_card_clicks,
                ui::game::check_pair_match,
                ui::game::end_turn_if_needed,
                ui::game::update_card_visibility,
                ui::game::update_replay_system,
                ui::game::update_game_logic,
                ui::game::update_ui_display,
                ui::game::update_card_visuals,
                ui::game::update_memory_board_visuals,
                crate::heartbeat::update_heartbeat,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), ui::cleanup::cleanup_ui)
            // Game over
            .add_systems(OnEnter(GameState::GameOver), ui::game_over::setup_game_over)
            .add_systems(Update, ui::game_over::handle_game_over.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), ui::cleanup::cleanup_ui)
        .run();
    }

/// Spawn the main camera once at startup to avoid ambiguity warnings
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


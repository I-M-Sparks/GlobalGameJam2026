use bevy::prelude::*;
use themathar_game::*;

#[derive(Resource)]
pub struct LocalPlayer {
    pub id: String,
    pub name: String,
    pub token: String,
}

#[derive(Resource)]
pub struct GameConfig {
    pub api_base_url: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<UIState>()
        .insert_resource(GameConfig {
            api_base_url: get_api_base_url(),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_ui_display,
        ))
        .run();
}

#[cfg(target_arch = "wasm32")]
fn get_api_base_url() -> String {
    use web_sys::window;
    
    if let Some(window) = window() {
        if let Ok(location) = window.location().href() {
            // Extract the base URL from current location
            if let Some(pos) = location.rfind('/') {
                return location[..pos].to_string();
            }
            return location;
        }
    }
    "http://localhost".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_api_base_url() -> String {
    "http://localhost".to_string()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_ui(commands, asset_server);
}

fn update_ui_display(
    mut status_query: Query<&mut Text, With<StatusText>>,
    mut timer_query: Query<&mut Text, (With<TimerText>, Without<StatusText>)>,
    mut queue_query: Query<&mut Text, (With<QueueText>, Without<StatusText>, Without<TimerText>)>,
    ui_state: Res<UIState>,
) {
    // Update status text
    if let Ok(mut text) = status_query.get_single_mut() {
        let status = if let Some(player_id) = &ui_state.player_id {
            if let Some(game_state) = &ui_state.current_game_state {
                if Some(player_id.clone()) == game_state.active_player_id {
                    "Status: YOU ARE THE ACTIVE PLAYER"
                } else if game_state.can_next_player_take_turn {
                    "Status: You can take your turn now!"
                } else {
                    "Status: Waiting for your turn"
                }
            } else {
                "Status: Loading..."
            }
        } else {
            "Status: Enter your name and join"
        };
        text.0 = status.to_string();
    }

    // Update timer text
    if let Ok(mut text) = timer_query.get_single_mut() {
        if let Some(game_state) = &ui_state.current_game_state {
            text.0 = format!("Time remaining: {} seconds", game_state.time_remaining);
        }
    }

    // Update queue text
    if let Ok(mut text) = queue_query.get_single_mut() {
        if let Some(game_state) = &ui_state.current_game_state {
            text.0 = format!("Queue length: {} players", game_state.queue_length);
        }
    }
}

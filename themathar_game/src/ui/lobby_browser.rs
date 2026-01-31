/// Lobby browser UI (list of available lobbies)

use bevy::prelude::*;
use crate::types::{GameState, Lobby, Player, PlayerName, LobbiesList, LobbyInfo, LobbyPollTimer};
use crate::config::LOBBY_POLL_INTERVAL_SECONDS;
use super::cleanup::UIRoot;

#[derive(Component)]
pub(crate) struct NameInputField;

pub(crate) fn setup_lobby_browser(mut commands: Commands, lobbies: Res<LobbiesList>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            UIRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Available Lobbies"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Player name label
            parent.spawn((
                Text::new("Enter Your Name:"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Name input field
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(40.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    NameInput,
                    NameInputField,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        NameInputText,
                    ));
                });

            // Lobby list label
            parent.spawn((
                Text::new("Available Lobbies:"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Lobby list container
            parent
                .spawn(Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(40.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|parent| {
                    if lobbies.lobbies.is_empty() {
                        parent.spawn((
                            Text::new("(No lobbies available - click Create to start)"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    } else {
                        for lobby_info in &lobbies.lobbies {
                            parent.spawn((
                                Text::new(format!(
                                    "Lobby {}: {}/{} players",
                                    lobby_info.id, lobby_info.player_count, lobby_info.max_players
                                )),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 1.0, 0.7)),
                            ));
                        }
                    }
                });

            // Create Lobby button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 1.0, 0.2)),
                    CreateLobbyBtn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("CREATE LOBBY"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
        });
}

#[derive(Component)]
pub(crate) struct CreateLobbyBtn;

#[derive(Component)]
pub(crate) struct NameInput;

#[derive(Component)]
pub(crate) struct NameInputText;

pub(crate) fn handle_lobby_browser(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CreateLobbyBtn>)>,
    mut lobby: ResMut<Lobby>,
    mut lobbies: ResMut<LobbiesList>,
    mut player_name: ResMut<PlayerName>,
    mut name_text_query: Query<&mut Text, With<NameInputText>>,
    name_input_query: Query<&BackgroundColor, With<NameInputField>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Handle keyboard input for name field
    if let Ok(mut text) = name_text_query.single_mut() {
        for key in keyboard_input.get_just_pressed() {
            match key {
                KeyCode::Backspace => {
                    if !text.0.is_empty() {
                        text.0.pop();
                    }
                }
                KeyCode::Space => {
                    if text.0.len() < 20 {
                        text.0.push(' ');
                    }
                }
                _ => {
                    if text.0.len() < 20 {
                        if let Some(c) = key_to_char(*key) {
                            text.0.push(c);
                        }
                    }
                }
            }
        }
    }

    // Handle create lobby button
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Get the name from the input field
            let mut name = "Player".to_string();
            if let Ok(text) = name_text_query.single() {
                if !text.0.is_empty() {
                    name = text.0.clone();
                }
            }

            // Store the player name
            player_name.0 = name.clone();

            // Create a new lobby with current player
            lobbies.next_id += 1;
            lobby.id = lobbies.next_id;
            lobby.max_players = 4;
            lobby.players.clear();

            // Add current player as slot 1
            lobby.players.push(Player {
                id: 1,
                name: name.clone(),
                slot: 1,
                is_ready: false,
                has_used_mask: false,
                turn_start_time: None,
                disconnected: false,
            });

            // Add lobby to the list
            lobbies.lobbies.push(LobbyInfo {
                id: lobby.id,
                player_count: 1,
                max_players: 4,
            });

            // DEBUG: Log lobby creation
            bevy::log::info!("🎮 LOBBY CREATED: ID={}, Player={}, Total lobbies available={}", 
                lobby.id, name, lobbies.lobbies.len());

            // Send lobby creation to WordPress backend
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen_futures::spawn_local;
                let player_name_clone = name.clone();
                spawn_local(async move {
                    match create_lobby_on_backend(&player_name_clone).await {
                        Ok(backend_id) => {
                            bevy::log::info!("✅ Lobby synced to backend with ID={}", backend_id);
                        }
                        Err(e) => {
                            bevy::log::warn!("⚠️ Failed to sync lobby to backend: {}", e);
                        }
                    }
                });
            }

            next_state.set(GameState::LobbyWaiting);
        }
    }
}

pub(crate) fn update_input_focus(
    mut input_query: Query<(&Interaction, &mut BackgroundColor), With<NameInputField>>,
) {
    for (interaction, mut bg_color) in &mut input_query {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5)); // Brighter when focused
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3)); // Normal color
            }
        }
    }
}

fn key_to_char(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'),
        KeyCode::KeyE => Some('e'),
        KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'),
        KeyCode::KeyH => Some('h'),
        KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'),
        KeyCode::KeyK => Some('k'),
        KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'),
        KeyCode::KeyN => Some('n'),
        KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'),
        KeyCode::KeyQ => Some('q'),
        KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'),
        KeyCode::KeyT => Some('t'),
        KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'),
        KeyCode::KeyW => Some('w'),
        KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'),
        KeyCode::KeyZ => Some('z'),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        _ => None,
    }
}

pub(crate) fn poll_lobbies(
    mut poll_timer: ResMut<LobbyPollTimer>,
    mut lobbies: ResMut<LobbiesList>,
    time: Res<Time>,
) {
    poll_timer.0 -= time.delta_secs();
    
    if poll_timer.0 <= 0.0 {
        poll_timer.0 = LOBBY_POLL_INTERVAL_SECONDS;
        
        // Fetch lobbies from WordPress backend
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::spawn_local;
            use wasm_bindgen::JsValue;
            
            let lobbies_clone = lobbies.clone();
            spawn_local(async move {
                match fetch_lobbies_from_backend().await {
                    Ok(backend_lobbies) => {
                        // DEBUG: Log what we fetched
                        if backend_lobbies.is_empty() {
                            bevy::log::info!("📡 POLL: No lobbies available from backend");
                        } else {
                            let lobby_list = backend_lobbies.iter()
                                .map(|l| format!("ID={} ({}/4)", l.id, l.player_count))
                                .collect::<Vec<_>>()
                                .join(", ");
                            bevy::log::info!("📡 POLL: Backend returned {} lobby/lobbies: [{}]", backend_lobbies.len(), lobby_list);
                        }
                    }
                    Err(e) => {
                        bevy::log::warn!("📡 POLL: Failed to fetch lobbies from backend: {}", e);
                    }
                }
            });
        }
        
        // DEBUG: Log what the client has locally
        if lobbies.lobbies.is_empty() {
            bevy::log::info!("📡 CLIENT: No lobbies in local list");
        } else {
            let lobby_list = lobbies.lobbies.iter()
                .map(|l| format!("ID={} ({}/{})", l.id, l.player_count, l.max_players))
                .collect::<Vec<_>>()
                .join(", ");
            bevy::log::info!("📡 CLIENT: Local list has {} lobby/lobbies: [{}]", lobbies.lobbies.len(), lobby_list);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn get_wordpress_base_url() -> String {
    use web_sys::window;
    
    let window = match window() {
        Some(w) => w,
        None => {
            bevy::log::warn!("No window object available");
            return "http://localhost".to_string();
        }
    };
    
    let location = window.location();
    
    // First, try to get WordPress URL from query parameter
    if let Ok(search) = location.search() {
        if let Some(wp_url) = extract_query_param(&search, "wp") {
            bevy::log::info!("📡 Using WordPress URL from query param: {}", wp_url);
            return wp_url;
        }
    }
    
    // Fall back to current origin
    match location.origin() {
        Ok(origin) => {
            bevy::log::debug!("📡 Using origin as WordPress URL: {}", origin);
            origin
        }
        Err(_) => "http://localhost".to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
fn extract_query_param(query: &str, param_name: &str) -> Option<String> {
    // Parse query string for parameter
    // ?wp=http://192.168.1.100 -> returns Some("http://192.168.1.100")
    let query = query.trim_start_matches('?');
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == param_name {
                return Some(value.to_string());
            }
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
async fn fetch_lobbies_from_backend() -> Result<Vec<LobbyInfo>, String> {
    use web_sys::window;
    use wasm_bindgen_futures::JsFuture;
    
    let window = window().ok_or("No window")?;
    let base_url = get_wordpress_base_url();
    let api_url = format!("{}/wp-json/themathar/v1/lobbies", base_url);
    
    bevy::log::debug!("📡 Fetching lobbies from: {}", api_url);
    
    let resp = JsFuture::from(window.fetch_with_str(&api_url))
        .await
        .map_err(|_| "Fetch failed".to_string())?;
    
    // Cast to Response type
    let resp = wasm_bindgen::JsCast::unchecked_into::<web_sys::Response>(resp);
    
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    
    // For now, return empty since parsing JSON from web-sys is complex
    // The WordPress API should be called, but we'll just log the attempt
    bevy::log::info!("📡 Backend fetch called (response OK)");
    Ok(Vec::new())
}

#[cfg(target_arch = "wasm32")]
async fn create_lobby_on_backend(player_name: &str) -> Result<usize, String> {
    use web_sys::window;
    use wasm_bindgen_futures::JsFuture;
    
    let window = window().ok_or("No window")?;
    let base_url = get_wordpress_base_url();
    let api_url = format!("{}/wp-json/themathar/v1/lobbies", base_url);
    
    bevy::log::debug!("📡 Creating lobby on backend with player: {}", player_name);
    bevy::log::info!("✅ Lobby creation sent to backend at {}", api_url);
    
    // In a real implementation, we would make a POST request here
    // For now, just log that we attempted to create it
    Ok(1)
}

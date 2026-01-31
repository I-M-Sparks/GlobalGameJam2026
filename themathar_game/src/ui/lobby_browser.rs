/// Lobby browser UI (list of available lobbies)

use bevy::prelude::*;
use crate::types::{GameState, Lobby, Player, PlayerName, LobbiesList, LobbyInfo, LobbyPollTimer};
use crate::config::LOBBY_POLL_INTERVAL_SECONDS;
use super::cleanup::UIRoot;

#[derive(Component)]
pub(crate) struct NameInputField;

pub(crate) fn setup_lobby_browser(mut commands: Commands, lobbies: Res<LobbiesList>) {
    commands.spawn(Camera2d::default());

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
        
        // In a real implementation, this would fetch lobbies from a backend server
        // For now, lobbies are added to the list when created locally
        // This function ensures the list stays in sync across clients
        // TODO: Make actual network call to fetch lobbies
    }
}

use super::cleanup::UIRoot;
use crate::config::TOTAL_PAIRS;
use crate::types::{Board, BoardState, Card, CardType, GameSession, GameState, Lobby, Player};
/// Player setup screen - enter 1-4 player names for hotseat multiplayer
use bevy::prelude::*;

const MAX_PLAYERS: usize = 4;
const MAX_NAME_LENGTH: usize = 20;

#[derive(Component)]
pub struct PlayerNameInput(pub usize); // Which player slot (0-3)

#[derive(Component)]
pub struct PlayerNameText(pub usize); // Text display for player name

#[derive(Component)]
pub struct StartGameButton;

/// Track the player names being entered
#[derive(Resource, Default)]
pub struct PlayerSetupState {
    pub names: [String; MAX_PLAYERS],
    pub active_input: usize, // Which input field is currently focused
}

pub(crate) fn setup_player_setup(
    mut commands: Commands,
    mut setup_state: ResMut<PlayerSetupState>,
) {
    setup_state.names = [String::new(), String::new(), String::new(), String::new()];
    setup_state.active_input = 0;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            UIRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ENTER PLAYER NAMES"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Player inputs container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..MAX_PLAYERS {
                        // Each player input row
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(20.0),
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|parent| {
                                // Player label
                                parent.spawn((
                                    Text::new(format!("Player {}:", i + 1)),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));

                                // Input field
                                parent
                                    .spawn((
                                        Node {
                                            width: Val::Px(300.0),
                                            height: Val::Px(50.0),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::FlexStart,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BorderColor::all(Color::srgb(1.0, 1.0, 1.0)),
                                        BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
                                        PlayerNameInput(i),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(""),
                                            TextFont {
                                                font_size: 24.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                            PlayerNameText(i),
                                        ));
                                    });
                            });
                    }
                });

            // Start button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                    StartGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START GAME"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });

            // Info text
            parent.spawn((
                Text::new("(At least 1 player required)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub(crate) fn handle_player_setup(
    mut commands: Commands,
    mut setup_state: ResMut<PlayerSetupState>,
    mut text_query: Query<(&PlayerNameText, &mut Text)>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lobby: ResMut<Lobby>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Handle keyboard input for active input field
    for key in keyboard_input.get_just_pressed() {
        let active_idx = setup_state.active_input;
        match key {
            KeyCode::Tab => {
                setup_state.active_input = (setup_state.active_input + 1) % MAX_PLAYERS;
            }
            KeyCode::Backspace => {
                if !setup_state.names[active_idx].is_empty() {
                    setup_state.names[active_idx].pop();
                }
            }
            KeyCode::Space => {
                if setup_state.names[active_idx].len() < MAX_NAME_LENGTH {
                    setup_state.names[active_idx].push(' ');
                }
            }
            _ => {
                if setup_state.names[active_idx].len() < MAX_NAME_LENGTH {
                    if let Some(c) = key_to_char(*key) {
                        setup_state.names[active_idx].push(c);
                    }
                }
            }
        }
    }

    // Update text display for all input fields
    for (text_idx, mut text) in text_query.iter_mut() {
        text.0 = setup_state.names[text_idx.0].clone();
    }

    // Check start button
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Create players from non-empty names
            lobby.players.clear();
            let mut player_count = 0;

            for name in &setup_state.names {
                if !name.is_empty() && player_count < MAX_PLAYERS {
                    lobby.players.push(Player {
                        id: player_count + 1,
                        name: name.clone(),
                        slot: player_count + 1,
                        is_ready: true,
                        has_used_mask: false,
                        turn_start_time: None,
                        disconnected: false,
                    });
                    player_count += 1;
                }
            }

            if player_count > 0 {
                lobby.max_players = player_count;
                lobby.id = 1;
                bevy::log::info!("🎮 Starting hotseat game with {} players", player_count);

                // Initialize the board with 16 cards (8 pairs)
                use rand::seq::SliceRandom;
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let mut cards = Vec::new();

                let excluded_pairs = ["A", "B", "C", "D", "E", "F", "G", "H"];
                let mut available_pairs: Vec<String> = Vec::new();

                if let Ok(entries) = std::fs::read_dir("assets/pairs") {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                let name = entry.file_name().to_string_lossy().to_string();
                                if !excluded_pairs.contains(&name.as_str()) {
                                    available_pairs.push(name);
                                }
                            }
                        }
                    }
                }

                if available_pairs.is_empty() {
                    bevy::log::warn!(
                        "No non A-H pair folders found in assets/pairs. Images may be missing."
                    );
                }

                available_pairs.shuffle(&mut rng);
                let selected_pairs: Vec<String> = if available_pairs.len() >= TOTAL_PAIRS {
                    available_pairs.into_iter().take(TOTAL_PAIRS).collect()
                } else {
                    bevy::log::warn!(
                        "Only {} non A-H pairs found. Reusing pairs to reach {}.",
                        available_pairs.len(),
                        TOTAL_PAIRS
                    );
                    let mut selected = available_pairs.clone();
                    while selected.len() < TOTAL_PAIRS && !available_pairs.is_empty() {
                        let idx = rng.gen_range(0..available_pairs.len());
                        selected.push(available_pairs[idx].clone());
                    }
                    selected
                };

                // Create 8 pairs (16 cards total), one Photo + one Art per pair
                for pair_id in 0..TOTAL_PAIRS {
                    cards.push(Card {
                        position: 0, // Will be set after shuffle
                        pair_id,
                        card_type: CardType::Photo,
                        is_face_up: false,
                        visibility_timer: 0.0,
                    });
                    cards.push(Card {
                        position: 0, // Will be set after shuffle
                        pair_id,
                        card_type: CardType::Art,
                        is_face_up: false,
                        visibility_timer: 0.0,
                    });
                }

                // Shuffle cards
                for i in (1..cards.len()).rev() {
                    let j = rng.gen_range(0..=i);
                    cards.swap(i, j);
                }

                // Set positions after shuffle
                for (idx, card) in cards.iter_mut().enumerate() {
                    card.position = idx;
                }

                let board = Board {
                    cards,
                    pair_folders: selected_pairs,
                };
                commands.insert_resource(board);

                // Initialize game session
                commands.insert_resource(GameSession {
                    lobby_id: 1,
                    active_player_slot: 1,
                    turn_number: 1,
                    game_time: 0.0,
                    winner_id: None,
                    board_state: BoardState::default(),
                    mask_used_this_turn: false,
                    turn_started_at: 0.0,
                    turn_timeout_at: 60.0,
                    grace_period_ends_at: 65.0,
                });

                next_state.set(GameState::Playing);
            }
        }
    }
}

pub(crate) fn update_input_focus(
    mut input_query: Query<(&PlayerNameInput, &mut BorderColor)>,
    setup_state: Res<PlayerSetupState>,
) {
    for (player_input, mut border) in input_query.iter_mut() {
        *border = if player_input.0 == setup_state.active_input {
            BorderColor::all(Color::srgb(0.0, 1.0, 1.0))
        } else {
            BorderColor::all(Color::srgb(1.0, 1.0, 1.0))
        };
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

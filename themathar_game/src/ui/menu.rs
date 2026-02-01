use super::cleanup::UIRoot;
use crate::config::TOTAL_PAIRS;
use crate::types::{Board, BoardState, Card, CardType, GameSession, GameState, Lobby, Player};
/// Main menu UI
use bevy::prelude::*;

pub(crate) fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            UIRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("THEMATHAR"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("The Mask That Remembers"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

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
                    BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
                    MenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START GAME"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Credits button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.6, 0.2)),
                    CreditsButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("CREDITS"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[derive(Component)]
pub(crate) struct MenuButton;

#[derive(Component)]
pub(crate) struct CreditsButton;

pub(crate) fn menu_input(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lobby: ResMut<Lobby>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Create 4 players with default names
            lobby.players.clear();

            for i in 0..4 {
                lobby.players.push(Player {
                    id: i + 1,
                    name: format!("Player {}", i + 1),
                    slot: i + 1,
                    is_ready: true,
                    has_used_mask: false,
                    turn_start_time: None,
                    disconnected: false,
                });
            }

            lobby.max_players = 4;
            lobby.id = 1;
            bevy::log::info!("🎮 Starting hotseat game with 4 players");

            // Initialize the board with 16 cards (8 pairs)
            use rand::seq::SliceRandom;
            use rand::Rng;
            let mut rng = rand::rng();
            let mut cards = Vec::new();

            let excluded_pairs = ["A", "B", "C", "D", "E", "F", "G", "H"];
            let mut available_pairs: Vec<String> = Vec::new();

            // Try multiple possible paths for assets
            let possible_paths = vec![
                "assets/pairs",
                "themathar_game/assets/pairs",
                "./assets/pairs",
            ];

            for path in &possible_paths {
                if let Ok(entries) = std::fs::read_dir(path) {
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
                    if !available_pairs.is_empty() {
                        break; // Found assets, stop trying other paths
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
                    let idx = rng.random_range(0..available_pairs.len());
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
                let j = rng.random_range(0..=i);
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

pub(crate) fn credits_button_input(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Credits);
        }
    }
}

/// Lobby waiting UI (players joining, ready button)

use bevy::prelude::*;
use crate::types::{GameState, Lobby, Board, GameSession};
use crate::config::*;
use crate::board;
use super::cleanup::UIRoot;

pub(crate) fn setup_lobby_waiting(mut commands: Commands) {
    commands.spawn(Camera2d::default());

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
                Text::new("Waiting for Players..."),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Player list
            parent
                .spawn(Node {
                    width: Val::Px(400.0),
                    height: Val::Px(200.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 1..=4 {
                        parent.spawn((
                            Text::new(format!("Slot {}: ", i)),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            LobbyPlayerSlot { slot: i },
                        ));
                    }
                });

            // Ready button
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
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    ReadyBtn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("MARK READY"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            // Start button (only visible if 4 players or all ready)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.8)),
                    StartBtn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START GAME"),
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
pub(crate) struct ReadyBtn;

#[derive(Component)]
pub(crate) struct StartBtn;

#[derive(Component)]
struct LobbyPlayerSlot {
    slot: usize,
}

pub(crate) fn handle_lobby_waiting(
    ready_query: Query<&Interaction, (Changed<Interaction>, With<ReadyBtn>)>,
    start_query: Query<&Interaction, (Changed<Interaction>, With<StartBtn>)>,
    mut lobby: ResMut<Lobby>,
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Handle ready button
    for interaction in &ready_query {
        if *interaction == Interaction::Pressed {
            if let Some(player) = lobby.players.first_mut() {
                player.is_ready = !player.is_ready;
            }
        }
    }

    // Handle start button
    for interaction in &start_query {
        if *interaction == Interaction::Pressed {
            // Initialize game
            *board = board::initialize_board();
            session.lobby_id = lobby.id;
            session.active_player_slot = 1; // Start with player 1
            session.turn_number = 0;
            session.game_time = 0.0;
            session.mask_used_this_turn = false;
            session.turn_started_at = 0.0;
            session.turn_timeout_at = TURN_TIME_LIMIT_SECONDS;
            session.grace_period_ends_at = TURN_TIME_LIMIT_SECONDS + TURN_TIMEOUT_GRACE_PERIOD_SECONDS;

            next_state.set(GameState::Playing);
        }
    }
}

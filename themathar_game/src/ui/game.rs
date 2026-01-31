/// Main game UI and systems

use bevy::prelude::*;
use crate::types::*;
use crate::config::*;
use crate::board;
use super::cleanup::UIRoot;

pub(crate) fn setup_game(
    mut commands: Commands,
    board: Res<Board>,
    _lobby: Res<Lobby>,
    mut local_player: ResMut<LocalPlayerSlot>,
) {
    // Set local player slot to 1 for now (single-player/local testing)
    // In real implementation, this comes from lobby join data
    local_player.0 = 1;

    commands.spawn(Camera2d::default());

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            UIRoot,
        ))
        .with_children(|parent| {
            // Player positions (clockwise: left, top, right, bottom)
            // Left player
            parent.spawn((
                Text::new("Player 1"),
                TextFont {
                    font_size: PLAYER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                PlayerNameDisplay,
            ));

            // Top player
            parent.spawn((
                Text::new("Player 2"),
                TextFont {
                    font_size: PLAYER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Px(20.0),
                    ..default()
                },
                PlayerNameDisplay,
            ));

            // Right player
            parent.spawn((
                Text::new("Player 3"),
                TextFont {
                    font_size: PLAYER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Percent(50.0),
                    ..default()
                },
                PlayerNameDisplay,
            ));

            // Bottom player
            parent.spawn((
                Text::new("Player 4"),
                TextFont {
                    font_size: PLAYER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    bottom: Val::Px(20.0),
                    ..default()
                },
                PlayerNameDisplay,
            ));

            // Central game board container
            parent
                .spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    ..default()
                })
                .with_children(|parent| {
                    // 4x4 Grid
                    parent
                        .spawn(Node {
                            display: Display::Grid,
                            grid_template_columns: vec![
                                GridTrack::px(CARD_WIDTH),
                                GridTrack::px(CARD_WIDTH),
                                GridTrack::px(CARD_WIDTH),
                                GridTrack::px(CARD_WIDTH),
                            ],
                            column_gap: Val::Px(CARD_GAP),
                            row_gap: Val::Px(CARD_GAP),
                            padding: UiRect::all(Val::Px(BOARD_PADDING)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Spawn 16 card entities
                            for position in 0..TOTAL_CARDS {
                                let card = board.card_at(position).unwrap();
                                parent
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(CARD_WIDTH),
                                            height: Val::Px(CARD_HEIGHT),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                                        CardVisual { position },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(if card.is_face_up {
                                                board::get_pair_name(card.pair_id)
                                            } else {
                                                "?".to_string()
                                            }),
                                            TextFont {
                                                font_size: 40.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                            }
                        });
                });

            // Bottom UI bar (timer, mask button, end turn button)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 0.0, 0.1)),
                ))
                .with_children(|parent| {
                    // Timer display
                    parent.spawn((
                        Text::new("60s"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.5, 0.2)),
                        TurnTimerDisplay,
                    ));

                    // Status text
                    parent.spawn((
                        Text::new("Active: Player 1"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        GameStatusDisplay,
                    ));

                    // Mask button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.2, 0.8)),
                            MaskButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("MASK"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // End turn button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.4, 0.4)),
                            EndTurnButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("END TURN"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub(crate) fn handle_card_clicks(
    interaction_query: Query<(&Interaction, &CardVisual), (Changed<Interaction>, With<Button>)>,
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    mut replay: ResMut<ReplaySystem>,
) {
    if replay.is_replaying {
        return;
    }

    // Don't allow flips if turn time is up
    if session.game_time >= session.turn_timeout_at {
        return;
    }

    for (interaction, card_visual) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Flip card if current player can flip
            if let Some(card) = board.card_at_mut(card_visual.position) {
                if !card.is_face_up {
                    card.is_face_up = true;
                    card.visibility_timer = CARD_FLIP_VISIBILITY_SECONDS;
                    session.board_state.current_turn_flips.push(card_visual.position);
                    session.board_state.last_flip_time = session.game_time;

                    // Record action for replay history
                    if replay.actions.len() >= ACTION_HISTORY_MAX_STORED {
                        replay.actions.remove(0);
                    }
                    let next_order = replay.actions.len() + 1;
                    replay.actions.push(CardFlipAction {
                        player_id: session.active_player_slot,
                        position: card_visual.position,
                        pair_id: card.pair_id,
                        card_type: card.card_type,
                        action_order: next_order,
                    });
                }
            }
        }
    }
}

pub(crate) fn update_card_visibility(
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    replay: Res<ReplaySystem>,
    time: Res<Time>,
) {
    if replay.is_replaying {
        return;
    }

    session.game_time += time.delta_secs();

    for card in board.cards.iter_mut() {
        if card.is_face_up && card.visibility_timer > 0.0 {
            card.visibility_timer -= time.delta_secs();
            if card.visibility_timer <= 0.0 && session.board_state.current_turn_flips.len() == 2 {
                // After 2 cards flip, check if they match
                card.is_face_up = false;
            }
        }
    }
}

pub(crate) fn update_game_logic(
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check win condition
    if board.all_flipped() {
        session.winner_id = Some(session.active_player_slot);
        next_state.set(GameState::GameOver);
        return;
    }

    // Check if turn should end (2 cards flipped and timer expired)
    if session.board_state.current_turn_flips.len() == 2 {
        let card1_pos = session.board_state.current_turn_flips[0];
        let card2_pos = session.board_state.current_turn_flips[1];

        if let (Some(card1), Some(card2)) = (board.card_at(card1_pos), board.card_at(card2_pos)) {
            if card1.visibility_timer <= 0.0 && card2.visibility_timer <= 0.0 {
                // Both cards have timed out
                if !board::is_matching_pair(card1, card2) {
                    // Mismatch - reset all cards and end turn
                    board::reset_all_cards(&mut board);
                    session.board_state.current_turn_flips.clear();
                    advance_turn(&mut session);
                } else {
                    // Match - leave cards face up, can flip more
                    session.board_state.current_turn_flips.clear();
                }
            }
        }
    }

    // Check if turn time is up
    if session.game_time >= session.grace_period_ends_at {
        // Grace period expired - must advance turn
        board::reset_all_cards(&mut board);
        session.board_state.current_turn_flips.clear();
        advance_turn(&mut session);
    } else if session.game_time >= session.turn_timeout_at {
        // Turn time exceeded, but still in grace period
        // Can't flip cards, but turn hasn't advanced yet
    }
}

fn advance_turn(session: &mut GameSession) {
    session.mask_used_this_turn = false;
    session.active_player_slot = if session.active_player_slot == 4 {
        1
    } else {
        session.active_player_slot + 1
    };
    session.turn_number += 1;

    // Set new turn timers
    session.turn_started_at = session.game_time;
    session.turn_timeout_at = session.game_time + TURN_TIME_LIMIT_SECONDS;
    session.grace_period_ends_at = session.game_time + TURN_TIME_LIMIT_SECONDS + TURN_TIMEOUT_GRACE_PERIOD_SECONDS;
}

pub(crate) fn update_ui_display(
    mut timer_query: Query<&mut Text, With<TurnTimerDisplay>>,
    mut button_query: Query<&mut BackgroundColor, With<MaskButton>>,
    session: Res<GameSession>,
    local_player: Res<LocalPlayerSlot>,
) {
    for mut text in timer_query.iter_mut() {
        let remaining = (session.turn_timeout_at - session.game_time).max(0.0);
        let status = if session.game_time >= session.turn_timeout_at {
            if session.game_time >= session.grace_period_ends_at {
                " [TURN OVER]".to_string()
            } else {
                " [TIMEOUT]".to_string()
            }
        } else {
            String::new()
        };
        text.0 = format!("{:.0}s{}", remaining, status);
    }

    // Disable mask button for non-active players or after timeout
    let is_active = local_player.0 == session.active_player_slot;
    let can_use_mask = is_active && session.game_time < session.turn_timeout_at;
    for mut bg_color in button_query.iter_mut() {
        if can_use_mask {
            *bg_color = BackgroundColor(Color::srgb(0.8, 0.2, 0.8)); // Active
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.4, 0.1, 0.4)); // Disabled
        }
    }
}

pub(crate) fn handle_mask_activation(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MaskButton>)>,
    board: Res<Board>,
    mut replay_board: ResMut<ReplayBoard>,
    mut replay: ResMut<ReplaySystem>,
    mut session: ResMut<GameSession>,
    mut commands: Commands,
    local_player: Res<LocalPlayerSlot>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            if replay.is_replaying || session.mask_used_this_turn {
                return;
            }

            // Only the local active player can activate mask
            if local_player.0 != session.active_player_slot {
                return;
            }

            if replay.actions.is_empty() {
                return;
            }

            // Prepare memory board
            replay_board.cards = board.cards.iter().map(|card| ReplayCard {
                position: card.position,
                pair_id: card.pair_id,
                card_type: card.card_type,
                is_face_up: false,
            }).collect();

            spawn_memory_board(&mut commands, &replay_board);

            // Start replay
            replay.is_replaying = true;
            replay.replay_timer = 0.0;
            replay.replay_index = 0;
            replay.after_timer = 0.0;
            replay.replay_progress = 0.0;
            replay.flip_interval = REPLAY_TOTAL_TIME_SECONDS / replay.actions.len().max(1) as f32;

            session.mask_used_this_turn = true;
        }
    }
}

pub(crate) fn update_replay_system(
    mut replay: ResMut<ReplaySystem>,
    mut replay_board: ResMut<ReplayBoard>,
    time: Res<Time>,
    mut commands: Commands,
    memory_board_query: Query<Entity, With<MemoryBoard>>,
    children_query: Query<&Children>,
    session: Res<GameSession>,
    local_player: Res<LocalPlayerSlot>,
) {
    if !replay.is_replaying {
        return;
    }

    // Only show replay to active player
    let is_active_player = local_player.0 == session.active_player_slot;

    replay.replay_timer += time.delta_secs();

    while replay.replay_index < replay.actions.len() && replay.replay_timer >= replay.flip_interval {
        let action = &replay.actions[replay.replay_index];
        if let Some(card) = replay_board.cards.iter_mut().find(|c| c.position == action.position) {
            card.is_face_up = true;
        }
        replay.replay_timer -= replay.flip_interval;
        replay.replay_index += 1;
    }

    if replay.replay_index >= replay.actions.len() {
        replay.after_timer += time.delta_secs();
        if replay.after_timer >= REPLAY_AFTER_TIME_SECONDS {
            replay.is_replaying = false;
            replay.replay_timer = 0.0;
            replay.after_timer = 0.0;

            for entity in memory_board_query.iter() {
                despawn_recursive(&mut commands, entity, &children_query);
            }
        }
    }
}

pub(crate) fn update_card_visuals(
    board: Res<Board>,
    card_query: Query<(&CardVisual, &Children)>,
    mut text_query: Query<&mut Text>,
    session: Res<GameSession>,
    local_player: Res<LocalPlayerSlot>,
) {
    let is_active_player = local_player.0 == session.active_player_slot;

    for (card_visual, children) in &card_query {
        if let Some(card) = board.card_at(card_visual.position) {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    // Only show card face if this is the active player's turn
                    text.0 = if is_active_player && card.is_face_up {
                        board::get_pair_name(card.pair_id)
                    } else {
                        "?".to_string()
                    };
                }
            }
        }
    }
}

pub(crate) fn update_memory_board_visuals(
    replay: Res<ReplaySystem>,
    replay_board: Res<ReplayBoard>,
    card_query: Query<(&MemoryCardVisual, &Children)>,
    mut text_query: Query<&mut Text>,
    session: Res<GameSession>,
    local_player: Res<LocalPlayerSlot>,
) {
    if !replay.is_replaying {
        return;
    }

    // Only update memory board visuals for active player
    let is_active_player = local_player.0 == session.active_player_slot;
    if !is_active_player {
        return;
    }

    for (card_visual, children) in &card_query {
        if let Some(card) = replay_board.cards.iter().find(|c| c.position == card_visual.position) {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = if card.is_face_up {
                        board::get_pair_name(card.pair_id)
                    } else {
                        "?".to_string()
                    };
                }
            }
        }
    }
}

fn spawn_memory_board(commands: &mut Commands, replay_board: &ReplayBoard) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            MemoryBoard,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::px(CARD_WIDTH),
                        GridTrack::px(CARD_WIDTH),
                        GridTrack::px(CARD_WIDTH),
                        GridTrack::px(CARD_WIDTH),
                    ],
                    column_gap: Val::Px(CARD_GAP),
                    row_gap: Val::Px(CARD_GAP),
                    padding: UiRect::all(Val::Px(BOARD_PADDING)),
                    ..default()
                })
                .with_children(|parent| {
                    for card in replay_board.cards.iter() {
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(CARD_WIDTH),
                                    height: Val::Px(CARD_HEIGHT),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                                MemoryCardVisual { position: card.position },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("?"),
                                    TextFont {
                                        font_size: 40.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                });
        });
}

fn despawn_recursive(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            despawn_recursive(commands, child, children_query);
        }
    }
    commands.entity(entity).despawn();
}

use super::cleanup::UIRoot;
use crate::config::*;
use crate::types::*;
/// Main game UI and systems
use bevy::prelude::*;
use std::path::Path;

#[derive(Resource, Default)]
pub struct PreloadedImages {
    pub handles: Vec<Handle<Image>>,
}

#[derive(Component)]
struct LoadingText;

fn get_pair_image_path(pair_folder: &str, card_type: CardType) -> String {
    let card_type_str = match card_type {
        CardType::Photo => "photo",
        CardType::Art => "art",
    };
    // Prefer existing on-disk extensions; fall back to .jpeg.
    let base = format!("pairs/{}/{}", pair_folder, card_type_str);
    let png_path = format!("assets/{}.png", base);
    if Path::new(&png_path).exists() {
        return format!("{}.png", base);
    }
    format!("{}.jpeg", base)
}

pub(crate) fn setup_loading(
    mut commands: Commands,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
) {
    // Preload all card images
    let mut preload_handles: Vec<Handle<Image>> = Vec::new();
    for pair_folder in board.pair_folders.iter() {
        for card_type in [CardType::Photo, CardType::Art] {
            let image_path = get_pair_image_path(pair_folder, card_type);
            preload_handles.push(asset_server.load(&image_path));
        }
    }
    commands.insert_resource(PreloadedImages {
        handles: preload_handles,
    });

    // Show loading UI
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
            parent.spawn((
                Text::new("LOADING ASSETS..."),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
                LoadingText,
            ));

            parent.spawn((
                Text::new("Please wait while images are loaded"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub(crate) fn check_loading_complete(
    preloaded: Res<PreloadedImages>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check if all handles are loaded
    let all_loaded = preloaded
        .handles
        .iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle.id()));

    if all_loaded {
        bevy::log::info!("✅ All assets loaded, starting game");
        next_state.set(GameState::Playing);
    }
}

pub(crate) fn setup_game(
    mut commands: Commands,
    board: Res<Board>,
    lobby: Res<Lobby>,
    asset_server: Res<AssetServer>,
) {
    let grid_width = CARD_WIDTH * 4.0 + CARD_GAP * 3.0 + BOARD_PADDING * 2.0;
    let grid_height = CARD_HEIGHT * 4.0 + CARD_GAP * 3.0 + BOARD_PADDING * 2.0;

    // Get player names from lobby (up to 4 players)
    let player_names: Vec<String> = (1..=4)
        .map(|slot| {
            lobby
                .player_at_slot(slot)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| format!("Player {}", slot))
        })
        .collect();

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
                Text::new(player_names[0].clone()),
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
                Text::new(player_names[1].clone()),
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
                Text::new(player_names[2].clone()),
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

            // Bottom player - positioned above the bottom UI bar (80px height + 20px margin)
            parent.spawn((
                Text::new(player_names[3].clone()),
                TextFont {
                    font_size: PLAYER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    bottom: Val::Px(100.0), // Above the 80px bottom UI bar
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
                    margin: UiRect {
                        left: Val::Px(-grid_width / 2.0),
                        right: Val::Auto,
                        top: Val::Px(-grid_height / 2.0),
                        bottom: Val::Auto,
                    },
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

                                // Construct image path based on pair_id and card_type
                                let pair_folder = board
                                    .pair_folders
                                    .get(card.pair_id)
                                    .map(|name| name.as_str())
                                    .unwrap_or("A");
                                let image_path = get_pair_image_path(pair_folder, card.card_type);

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
                                        // Image - shown when face-up (absolute positioning to fill card)
                                        parent.spawn((
                                            ImageNode::new(asset_server.load(&image_path)),
                                            Node {
                                                position_type: PositionType::Absolute,
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                left: Val::Px(0.0),
                                                top: Val::Px(0.0),
                                                ..default()
                                            },
                                            Visibility::Hidden, // Start hidden, will be shown when face-up
                                        ));

                                        // "?" text - shown when face-down (absolute positioning centered)
                                        parent.spawn((
                                            Text::new("?".to_string()),
                                            TextFont {
                                                font_size: 40.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            Node {
                                                position_type: PositionType::Absolute,
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                left: Val::Px(0.0),
                                                top: Val::Px(0.0),
                                                ..default()
                                            },
                                            Visibility::Visible, // Start visible (face-down)
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
            // Only allow up to 2 cards per turn
            if session.board_state.current_turn_flips.len() >= 2 {
                return;
            }

            // Flip card if current player can flip
            if let Some(card) = board.card_at_mut(card_visual.position) {
                if !card.is_face_up {
                    card.is_face_up = true;
                    card.visibility_timer = CARD_FLIP_VISIBILITY_SECONDS;
                    session
                        .board_state
                        .current_turn_flips
                        .push(card_visual.position);
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

pub(crate) fn check_pair_match(board: Res<Board>, mut session: ResMut<GameSession>) {
    // Only check if exactly 2 cards are flipped
    if session.board_state.current_turn_flips.len() != 2 {
        return;
    }

    // Don't check if already checked
    if session.board_state.pair_match_result.is_some() {
        return;
    }

    let pos1 = session.board_state.current_turn_flips[0];
    let pos2 = session.board_state.current_turn_flips[1];

    if let (Some(card1), Some(card2)) = (board.card_at(pos1), board.card_at(pos2)) {
        let is_match = card1.pair_id == card2.pair_id;
        session.board_state.pair_match_result = Some(is_match);

        // Set delay before turn-end behavior (allow time for visual feedback)
        session.board_state.pair_check_delay = if is_match { 0.5 } else { 1.0 };
    }
}

pub(crate) fn end_turn_if_needed(
    mut _board: ResMut<Board>,
    mut _session: ResMut<GameSession>,
    mut _next_state: ResMut<NextState<GameState>>,
) {
    // All turn logic is consolidated in update_game_logic - this function is disabled
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

    // Only decrement visibility timers for cards that are part of the current turn flips
    // and don't match (we'll handle matched pairs separately in update_game_logic)
    for &pos in &session.board_state.current_turn_flips {
        if let Some(card) = board.card_at_mut(pos) {
            if card.is_face_up && card.visibility_timer > 0.0 {
                card.visibility_timer -= time.delta_secs();
            }
        }
    }
}

pub(crate) fn update_game_logic(
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Check win condition: all cards are face-up (all pairs found)
    if board.all_flipped() {
        session.winner_id = Some(session.active_player_slot);
        next_state.set(GameState::GameOver);
        return;
    }

    // If exactly 2 cards have been flipped and we have a match result
    if session.board_state.current_turn_flips.len() == 2 {
        let card1_pos = session.board_state.current_turn_flips[0];
        let card2_pos = session.board_state.current_turn_flips[1];

        if let (Some(card1), Some(card2)) = (board.card_at(card1_pos), board.card_at(card2_pos)) {
            let is_match = session.board_state.pair_match_result.unwrap_or(false);

            println!(
                "DEBUG: Flipped 2 cards - Timer1={:.2}, Timer2={:.2}, Match={:?}",
                card1.visibility_timer, card2.visibility_timer, is_match
            );

            // Case 1: Cards matched - keep them face-up and clear turn flips so player can flip more
            if is_match {
                println!("DEBUG: Match found! Keeping cards face-up, player continues their turn");
                session.board_state.current_turn_flips.clear();
                session.board_state.pair_match_result = None;
                return;
            }

            // Case 2: Cards don't match - wait for visibility timer to expire, then flip them back
            if card1.visibility_timer <= 0.0 && card2.visibility_timer <= 0.0 {
                println!("DEBUG: Non-match detected, visibility expired. Flipping cards back down and advancing turn");
                // Flip these two cards back face-down
                if let Some(card) = board.card_at_mut(card1_pos) {
                    card.is_face_up = false;
                }
                if let Some(card) = board.card_at_mut(card2_pos) {
                    card.is_face_up = false;
                }
                session.board_state.current_turn_flips.clear();
                session.board_state.pair_match_result = None;
                advance_turn(&mut board, &mut session);
                return;
            }
        }
    }

    // Option 3: Overall turn time is up (grace period expired) - end turn immediately
    if session.game_time >= session.grace_period_ends_at {
        println!("DEBUG: Turn time expired (grace period). Ending turn and flipping all cards");

        // Flip back any unmatched pairs that were just revealed
        if session.board_state.current_turn_flips.len() == 2
            && session.board_state.pair_match_result == Some(false)
        {
            for &pos in &session.board_state.current_turn_flips {
                if let Some(card) = board.card_at_mut(pos) {
                    card.is_face_up = false;
                }
            }
        }

        session.board_state.current_turn_flips.clear();
        session.board_state.pair_match_result = None;
        advance_turn(&mut board, &mut session);
    }
}

fn advance_turn(board: &mut ResMut<Board>, session: &mut GameSession) {
    // Flip ALL cards back to face-down when advancing to next player
    println!("DEBUG: Flipping all cards to face-down for next player");
    for card in board.cards.iter_mut() {
        card.is_face_up = false;
        card.visibility_timer = 0.0;
    }

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
    session.grace_period_ends_at =
        session.game_time + TURN_TIME_LIMIT_SECONDS + TURN_TIMEOUT_GRACE_PERIOD_SECONDS;
}

pub(crate) fn update_ui_display(
    mut timer_query: Query<&mut Text, With<TurnTimerDisplay>>,
    mut status_query: Query<&mut Text, (With<GameStatusDisplay>, Without<TurnTimerDisplay>)>,
    mut button_query: Query<&mut BackgroundColor, With<MaskButton>>,
    session: Res<GameSession>,
    _local_player: Res<LocalPlayerSlot>,
    lobby: Res<Lobby>,
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

    // Update status display with active player name
    for mut text in status_query.iter_mut() {
        let active_player_name = lobby
            .player_at_slot(session.active_player_slot)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| format!("Player {}", session.active_player_slot));
        text.0 = format!("Active: {}", active_player_name);
    }

    // Update mask button state - available once per turn before timeout
    let can_use_mask = !session.mask_used_this_turn && session.game_time < session.turn_timeout_at;
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
    asset_server: Res<AssetServer>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            if replay.is_replaying || session.mask_used_this_turn {
                return;
            }

            // In hotseat mode, any player can activate mask during their turn

            if replay.actions.is_empty() {
                return;
            }

            // Prepare memory board
            replay_board.cards = board
                .cards
                .iter()
                .map(|card| ReplayCard {
                    position: card.position,
                    pair_id: card.pair_id,
                    card_type: card.card_type,
                    is_face_up: false,
                })
                .collect();

            spawn_memory_board(&mut commands, &replay_board, &board, &asset_server);

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
    _session: Res<GameSession>,
) {
    if !replay.is_replaying {
        return;
    }

    // In hotseat mode, all players see the replay

    replay.replay_timer += time.delta_secs();

    while replay.replay_index < replay.actions.len() && replay.replay_timer >= replay.flip_interval
    {
        let action = &replay.actions[replay.replay_index];
        if let Some(card) = replay_board
            .cards
            .iter_mut()
            .find(|c| c.position == action.position)
        {
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
    mut visibility_query: Query<&mut Visibility>,
) {
    // In hotseat mode, all players see the same board

    for (card_visual, children) in &card_query {
        if let Some(card) = board.card_at(card_visual.position) {
            // Toggle visibility of child elements (image and "?" text)
            for (i, child) in children.iter().enumerate() {
                if let Ok(mut vis) = visibility_query.get_mut(child) {
                    match i {
                        0 => {
                            // First child is ImageNode - show when face-up
                            *vis = if card.is_face_up {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            };
                        }
                        1 => {
                            // Second child is "?" text - show when face-down
                            *vis = if card.is_face_up {
                                Visibility::Hidden
                            } else {
                                Visibility::Visible
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub(crate) fn update_memory_board_visuals(
    replay: Res<ReplaySystem>,
    replay_board: Res<ReplayBoard>,
    card_query: Query<(&MemoryCardVisual, &Children)>,
    mut visibility_query: Query<&mut Visibility>,
    _session: Res<GameSession>,
) {
    if !replay.is_replaying {
        return;
    }

    // In hotseat mode, all players can see the replay

    for (card_visual, children) in &card_query {
        if let Some(card) = replay_board
            .cards
            .iter()
            .find(|c| c.position == card_visual.position)
        {
            for (i, child) in children.iter().enumerate() {
                if let Ok(mut vis) = visibility_query.get_mut(child) {
                    match i {
                        0 => {
                            // First child is ImageNode - show when face-up
                            *vis = if card.is_face_up {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            };
                        }
                        1 => {
                            // Second child is "?" text - show when face-down
                            *vis = if card.is_face_up {
                                Visibility::Hidden
                            } else {
                                Visibility::Visible
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn spawn_memory_board(
    commands: &mut Commands,
    replay_board: &ReplayBoard,
    board: &Board,
    asset_server: &AssetServer,
) {
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
                        // Construct image path based on pair_id
                        let pair_folder = board
                            .pair_folders
                            .get(card.pair_id)
                            .map(|name| name.as_str())
                            .unwrap_or("A");
                        let image_path = get_pair_image_path(pair_folder, card.card_type);

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
                                MemoryCardVisual {
                                    position: card.position,
                                },
                            ))
                            .with_children(|parent| {
                                // Image - shown when face-up (absolute positioning to fill card)
                                parent.spawn((
                                    ImageNode::new(asset_server.load(&image_path)),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        ..default()
                                    },
                                    Visibility::Hidden, // Will be shown based on card state
                                ));

                                // "?" text - shown when face-down (absolute positioning centered)
                                parent.spawn((
                                    Text::new("?"),
                                    TextFont {
                                        font_size: 40.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        ..default()
                                    },
                                    Visibility::Visible,
                                ));
                            });
                    }
                });
        });
}

fn despawn_recursive(commands: &mut Commands, entity: Entity, children_query: &Query<&Children>) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            despawn_recursive(commands, child, children_query);
        }
    }
    commands.entity(entity).despawn();
}

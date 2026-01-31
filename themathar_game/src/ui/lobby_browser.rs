/// Lobby browser UI (list of available lobbies)

use bevy::prelude::*;
use crate::types::{GameState, Lobby, Player};
use super::cleanup::UIRoot;

pub(crate) fn setup_lobby_browser(mut commands: Commands) {
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

            // Lobby list container
            parent
                .spawn(Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(60.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|parent| {
                    // TODO: Populate with lobbies from backend
                    parent.spawn((
                        Text::new("(No lobbies available - click Create to start)"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
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

pub(crate) fn handle_lobby_browser(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CreateLobbyBtn>)>,
    mut lobby: ResMut<Lobby>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Create a new lobby with current player
            lobby.id = 1; // TODO: Get from backend
            lobby.max_players = 4;
            lobby.players.clear();

            // Add current player as slot 1
            lobby.players.push(Player {
                id: 1,
                name: "Player".to_string(), // TODO: Get from user input
                slot: 1,
                is_ready: false,
                has_used_mask: false,
                turn_start_time: None,
                disconnected: false,
            });

            next_state.set(GameState::LobbyWaiting);
        }
    }
}

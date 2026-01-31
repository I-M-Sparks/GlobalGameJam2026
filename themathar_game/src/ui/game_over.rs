/// Game over screen

use bevy::prelude::*;
use crate::types::{GameState, GameSession, Lobby};
use super::cleanup::UIRoot;

pub(crate) fn setup_game_over(
    mut commands: Commands,
    session: Res<GameSession>,
    lobby: Res<Lobby>,
) {
    commands.spawn(Camera2d::default());

    let winner_name = session
        .winner_id
        .and_then(|slot| lobby.player_at_slot(slot))
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
            UIRoot,
        ))
        .with_children(|parent| {
            // Winner announcement
            parent.spawn((
                Text::new(format!("{} WINS!", winner_name)),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
            ));

            // Play again button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    PlayAgainBtn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY AGAIN"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });

            // Leave button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    LeaveBtn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("LEAVE"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[derive(Component)]
pub(crate) struct PlayAgainBtn;

#[derive(Component)]
pub(crate) struct LeaveBtn;

pub(crate) fn handle_game_over(
    play_again_query: Query<&Interaction, (Changed<Interaction>, With<PlayAgainBtn>)>,
    leave_query: Query<&Interaction, (Changed<Interaction>, With<LeaveBtn>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &play_again_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::LobbyWaiting);
        }
    }

    for interaction in &leave_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}

/// Main menu UI

use bevy::prelude::*;
use crate::types::GameState;
use super::cleanup::UIRoot;

pub(crate) fn setup_menu(mut commands: Commands) {
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
        });
}

#[derive(Component)]
pub(crate) struct MenuButton;

pub(crate) fn menu_input(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::LobbyBrowser);
        }
    }
}

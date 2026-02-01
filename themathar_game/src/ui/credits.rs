/// Credits UI

use bevy::prelude::*;
use crate::types::GameState;
use super::cleanup::UIRoot;

pub(crate) fn setup_credits(mut commands: Commands) {
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
                Text::new("CREDITS"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
            ));

            // Credits content
            parent.spawn((
                Text::new(
                    "Photo Material Provided By:\n\n\
                    Haus der Geschichte Baden-Württemberg\n\
                    Deutsches Fleischerei Museum Böblingen\n\
                    Stadtmuseum Hornmoldhaus Bietigheim-Bissingen"
                ),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Back button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                    BackButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("BACK"),
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
pub(crate) struct BackButton;

pub(crate) fn credits_input(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}

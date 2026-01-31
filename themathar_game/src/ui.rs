use bevy::prelude::*;
use crate::api::GameStateResponse;

#[derive(Component)]
pub struct UIButton;

#[derive(Component)]
pub struct JoinButton;

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct PlayerNameInput;

#[derive(Component)]
pub struct StatusText;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct QueueText;

#[derive(Resource)]
pub struct UIState {
    pub player_name_input: String,
    pub show_name_input: bool,
    pub current_game_state: Option<GameStateResponse>,
    pub player_id: Option<String>,
    pub player_token: Option<String>,
    pub error_message: Option<String>,
    pub is_joining: bool,
}

impl Default for UIState {
    fn default() -> Self {
        UIState {
            player_name_input: String::new(),
            show_name_input: true,
            current_game_state: None,
            player_id: None,
            player_token: None,
            error_message: None,
            is_joining: false,
        }
    }
}

pub fn create_ui(
    mut commands: Commands,
) {
    // Root camera
    commands.spawn(Camera2d::default());

    // Main container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Themathar - Multiplayer Game"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
            ));

            // Player name input
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    PlayerNameInput,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Enter your name:"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(40.0),
                            border: UiRect::all(Val::Px(2.0)),
                            padding: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.95, 0.95, 0.95)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Type your name..."),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        ));
                    });
                });

            // Status display
            parent.spawn((
                Text::new("Status: Waiting for player name"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                StatusText,
            ));

            // Timer display
            parent.spawn((
                Text::new("Time remaining: --"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TimerText,
            ));

            // Queue display
            parent.spawn((
                Text::new("Queue length: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                QueueText,
            ));

            // Join queue button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.2)),
                    JoinButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Join Queue"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });

            // End turn button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.7, 0.2, 0.2)),
                    EndTurnButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("End Turn"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });

            // Error message display
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
        });
}

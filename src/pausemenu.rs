use crate::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::states::AppState;
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_menu.in_schedule(OnEnter(AppState::Paused)));
        app.add_system(process_menu.in_set(OnUpdate(AppState::Paused)));
        app.add_system(unpause.in_set(OnUpdate(AppState::Paused)));
        app.add_system(destroy_menu.in_schedule(OnExit(AppState::Paused)));
    }
}

#[derive(Resource)]
struct MenuEntities {
    pause_menu: Entity,
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pause_menu = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font: asset_server.load("fonts/pixelsplitter.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        // center button
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Press Space to Unpause",
                        TextStyle {
                            font: asset_server.load("fonts/pixelsplitter.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 0.0, 0.0),
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuEntities { pause_menu });
}

fn process_menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::Menu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn unpause(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

fn destroy_menu(mut commands: Commands, menu_data: Res<MenuEntities>) {
    commands.entity(menu_data.pause_menu).despawn_recursive();
}

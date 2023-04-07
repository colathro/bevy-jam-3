use crate::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::gameplay::{Bullet, Enemy, EnemySpawner, Player};
use crate::states::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_system(create_menu.in_schedule(OnEnter(AppState::Menu)));
        app.add_system(spawn_background.in_schedule(OnEnter(AppState::Menu)));
        app.add_system(spawn_map.in_schedule(OnEnter(AppState::Menu)));
        app.add_system(destroy_world_things.in_schedule(OnEnter(AppState::Menu)));
        app.add_system(process_menu.in_set(OnUpdate(AppState::Menu)));
        app.add_system(destroy_menu.in_schedule(OnExit(AppState::Menu)));
    }
}

#[derive(Resource)]
struct MenuEntities {
    start_button: Entity,
}

#[derive(Component)]
struct Map;

#[derive(Component)]
struct MainMenuBackground;

fn spawn_map(mut commands: Commands, map_query: Query<&Map>, asset_server: Res<AssetServer>) {
    for _ in map_query.iter() {
        return;
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/map.png"),
            ..default()
        })
        .insert(Map)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

fn spawn_background(
    mut commands: Commands,
    background_query: Query<&MainMenuBackground>,
    asset_server: Res<AssetServer>,
) {
    for _ in background_query.iter() {
        return;
    }

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/Title.png"),
            ..default()
        })
        .insert(MainMenuBackground)
        .insert(TransformBundle::from(
            Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3 {
                x: 2.6,
                y: 2.5,
                z: 0.0,
            }),
        ));
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = commands
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
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/pixelsplitter.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent.spawn(TextBundle::from_section(
                "WASD to Move.",
                TextStyle {
                    font: asset_server.load("fonts/pixelsplitter.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
            parent.spawn(TextBundle::from_section(
                "R to Reload",
                TextStyle {
                    font: asset_server.load("fonts/pixelsplitter.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Esc to Pause",
                TextStyle {
                    font: asset_server.load("fonts/pixelsplitter.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();
    commands.insert_resource(MenuEntities { start_button });
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
                next_state.set(AppState::Cutscene);
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

fn destroy_menu(
    mut commands: Commands,
    menu_data: Res<MenuEntities>,
    background_query: Query<(&MainMenuBackground, Entity)>,
) {
    commands.entity(menu_data.start_button).despawn_recursive();
    for (_, entity) in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn destroy_world_things(
    mut commands: Commands,
    player_query: Query<(&Player, Entity)>,
    enemy_query: Query<(&Enemy, Entity)>,
    enemy_spawner_query: Query<(&EnemySpawner, Entity)>,
    bullet_query: Query<(&Bullet, Entity)>,
) {
    for (_, entity) in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (_, entity) in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (_, entity) in enemy_spawner_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (_, entity) in bullet_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

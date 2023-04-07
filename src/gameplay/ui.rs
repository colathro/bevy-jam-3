use crate::states::AppState;
use bevy::prelude::*;
use instant::{Duration, Instant};

use super::{EnemySpawner, Player};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_menu.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(update_shell_count.in_set(OnUpdate(AppState::InGame)));
        app.add_system(update_score.in_set(OnUpdate(AppState::InGame)));
        app.add_system(update_hp.in_set(OnUpdate(AppState::InGame)));
        app.add_system(handle_stage_change.in_set(OnUpdate(AppState::InGame)));
        app.add_system(handle_remove_stage.in_set(OnUpdate(AppState::InGame)));
        app.add_system(destroy_menu.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Resource)]
struct MenuEntities {
    hp: Entity,
    bullets: Entity,
    score: Entity,
}

#[derive(Component)]
struct Bullets;

#[derive(Component)]
struct HP;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Stage {
    shown: Instant,
}

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bullets = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Shells: ",
                    TextStyle {
                        font: asset_server.load("fonts/pixelsplitter.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                ))
                .insert(Bullets);
        })
        .id();

    let hp = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "HP: ",
                    TextStyle {
                        font: asset_server.load("fonts/pixelsplitter.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                ))
                .insert(HP);
        })
        .id();

    let score = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("fonts/pixelsplitter.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                ))
                .insert(Score);
        })
        .id();
    commands.insert_resource(MenuEntities { bullets, hp, score });
}

fn update_shell_count(
    player_query: Query<&Player>,
    mut bullet_text_query: Query<&mut Text, With<Bullets>>,
) {
    if let Ok(player) = player_query.get_single() {
        let value = player.shells;
        for mut text in &mut bullet_text_query {
            text.sections[0].value = format!("Shells: {value:.2}");
        }
    }
}

fn update_score(
    enemy_spawner_query: Query<&EnemySpawner>,
    mut score_text_query: Query<&mut Text, With<Score>>,
) {
    if let Ok(enemy_spawner) = enemy_spawner_query.get_single() {
        let value = enemy_spawner.score;
        for mut text in &mut score_text_query {
            text.sections[0].value = format!("Score: {value:.2}");
        }
    }
}

fn update_hp(player_query: Query<&Player>, mut hp_text_query: Query<&mut Text, With<HP>>) {
    if let Ok(player) = player_query.get_single() {
        let value = player.health;
        for mut text in &mut hp_text_query {
            text.sections[0].value = format!("HP: {value:.2}");
        }
    }
}

fn destroy_menu(
    mut commands: Commands,
    menu_data: Res<MenuEntities>,
    stage_text_query: Query<(&Stage, Entity)>,
) {
    commands.entity(menu_data.bullets).despawn_recursive();
    commands.entity(menu_data.hp).despawn_recursive();
    commands.entity(menu_data.score).despawn_recursive();

    for (_, entity) in stage_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_stage_change(
    mut commands: Commands,
    enemy_spawner_query: Query<&EnemySpawner>,
    mut last_stage: Local<u32>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(enemy_spawner) = enemy_spawner_query.get_single() {
        if *last_stage != enemy_spawner.stage {
            *last_stage = enemy_spawner.stage;
            let value = *last_stage;
            commands
                .spawn(NodeBundle {
                    style: Style {
                        // center button
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            format!("Starting Stage {value:.2}"),
                            TextStyle {
                                font: asset_server.load("fonts/pixelsplitter.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                            },
                        ))
                        .insert(Stage {
                            shown: Instant::now(),
                        });
                });
        }
    }
}

fn handle_remove_stage(mut commands: Commands, stage_text_query: Query<(&Stage, Entity)>) {
    for (stage, entity) in stage_text_query.iter() {
        if Instant::now() > stage.shown + Duration::from_secs(2) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

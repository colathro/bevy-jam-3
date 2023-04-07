use crate::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::states::AppState;
use bevy::prelude::*;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_menu.in_schedule(OnEnter(AppState::Scoreboard)));
        app.add_system(process_menu.in_set(OnUpdate(AppState::Scoreboard)));
        app.add_system(destroy_menu.in_schedule(OnExit(AppState::Scoreboard)));
    }
}

#[derive(Resource)]
struct ScoreBoardEntities {
    score_menu: Entity,
}

#[derive(Component)]
struct ScoreBoardSprite;

fn create_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let score_menu = commands
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
                        "You fuckin died.",
                        TextStyle {
                            font: asset_server.load("fonts/pixelsplitter.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 0.0, 0.0),
                        },
                    ));
                });
        })
        .id();
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/Baby.png"),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3 {
                    x: -240.0,
                    y: -120.0,
                    z: 2.0,
                },
                scale: Vec3::new(6.0, 6.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(ScoreBoardSprite);
    commands.insert_resource(ScoreBoardEntities { score_menu });
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

fn destroy_menu(
    mut commands: Commands,
    menu_data: Res<ScoreBoardEntities>,
    scoreboard_sprite_query: Query<(&ScoreBoardSprite, Entity)>,
) {
    commands.entity(menu_data.score_menu).despawn_recursive();
    for (_, entity) in scoreboard_sprite_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
